use std::{collections::{HashMap, VecDeque}, fs, vec};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Default)]
struct Schema {
    modules: HashMap<String, Module>
}

impl Schema {
    fn get_cycle(&mut self) -> [i64; 2] {
        let mut pcount = [0, 0];

        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
    
        while let Some((sender, name, pulse)) = queue.pop_front() {
            pcount[pulse as usize] += 1;
    
            if let Some(Module { name, mod_type, connects }) = self.modules.get_mut(&name) {
                match mod_type {
                    Type::Broadcaster => {
                        for to in connects {
                            queue.push_back((name.to_string(), to.to_string(), pulse))
                        }
                    },
                    Type::FlipFlop(state) if pulse == Pulse::Low => {
                        *state = !*state;
                        
                        let pulse = Pulse::to_pulse(*state);
                        for to in  connects {
                            queue.push_back((name.to_string(), to.to_string(), pulse))
                        }
                    },
                    Type::Conjuction(connections) => {
                        connections.insert(sender.to_string(), pulse);

                        let is_high = !connections.iter().all(|x| *x.1 == Pulse::High);
                        let pulse = Pulse::to_pulse(is_high);
                        for to in connects {
                            queue.push_back((name.to_string(), to.to_string(), pulse))
                        }
                    },
                    _ => ()
                }
            }
        }

        pcount
    }

    fn get_cycles_to_target(&mut self, target: (String, String, Pulse)) -> i64 {
        let mut cycles = 1;

        let mut queue = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
    
        while let Some((sender, name, pulse)) = queue.pop_front() {    
            if let Some(Module { name, mod_type, connects }) = self.modules.get_mut(&name) {
                if sender == target.0 && *name == target.1 &&  pulse == target.2 {
                    return cycles;
                }

                match mod_type {
                    Type::Broadcaster => {
                        for to in connects {
                            queue.push_back((name.to_string(), to.to_string(), pulse))
                        }
                    },
                    Type::FlipFlop(state) if pulse == Pulse::Low => {
                        *state = !*state;
                        
                        let pulse = Pulse::to_pulse(*state);
                        for to in  connects {
                            queue.push_back((name.to_string(), to.to_string(), pulse))
                        }
                    },
                    Type::Conjuction(connections) => {
                        connections.insert(sender.to_string(), pulse);

                        let is_high = !connections.iter().all(|x| *x.1 == Pulse::High);
                        let pulse = Pulse::to_pulse(is_high);
                        for to in connects {
                            queue.push_back((name.to_string(), to.to_string(), pulse))
                        }
                    },
                    _ => ()
                }
            }

            if queue.len() == 0 {
                cycles += 1;
                queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
            }
        }

        panic!("Should return result")
    }
}

#[derive(Clone)]
struct Module {
    name: String,
    mod_type: Type,
    connects: Vec<String>,
}

#[derive(Clone)]
enum Type {
    FlipFlop(bool),
    Conjuction(HashMap<String, Pulse>),
    Broadcaster
}

impl Type {
    const FLIPFLOP: char = '%';
    const CONJUCTION: char = '&';
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Pulse {
    Low, 
    High
}

impl Pulse {
    fn to_pulse(state: bool) -> Pulse {
        match state {
            false => Pulse::Low,
            true => Pulse::High
        }
    }
}

fn main() -> Result<()> {
    let schema = parse()?;

    part1(schema.clone(), 1000)?;
    part2(&schema)?;

    return Ok(());
}

fn parse() -> Result<Schema> {
    let input = fs::read_to_string("input.txt")?;
    let mut conjunctions = HashMap::<String, HashMap<String, Pulse>>::new();

    let mut modules = input.lines().map(|line| {
        let (module, connections) = line.split_once("->").unwrap();
        let module = module.trim();
        let connects = connections.split(',')
            .map(|c| c.trim().to_string())
            .collect::<Vec<_>>();

        let (mod_type, name) = match module.chars().nth(0).unwrap() {
            Type::FLIPFLOP => (Type::FlipFlop(false), module.trim_start_matches(Type::FLIPFLOP)),
            Type::CONJUCTION => {
                let name = module.trim_start_matches(Type::CONJUCTION);
                conjunctions.insert(name.to_string(), HashMap::new());
                (Type::Conjuction(HashMap::new()), name)
            },
            _ => (Type::Broadcaster, module)
        };

        (name.to_string(), Module { name: name.to_string(), mod_type, connects})
    }).collect::<HashMap<String, Module>>();

    for (name, module) in &modules {
        for connect in &module.connects {
            if let Some(conj) = conjunctions.get_mut(connect) {
                conj.insert(name.to_string(), Pulse::Low);
            }
        }
    }

    for (name, module) in modules.iter_mut() {
        if let Some(conj) = conjunctions.get(name) {
            module.mod_type = Type::Conjuction(conj.clone());
        }
    }

    Ok(Schema { modules })
}

fn part1(mut schema: Schema, cycles: i64) -> Result<()> {
    let mut pulses = [0, 0];
    for _ in 0..cycles {
        let cyc_puls = schema.get_cycle();
        pulses[0] += cyc_puls[0];
        pulses[1] += cyc_puls[1];
    }

    println!("Part 1 answer: {}", pulses.iter().product::<i64>());
    return Ok(());
}

fn part2(schema: &Schema) -> Result<()> {

    let nodes_to_rx = vec![ "cl", "rp", "lb", "nj" ];
    let mut multipliers = Vec::<i64>::new();

    for target in nodes_to_rx {
        let mut clean_schema = schema.clone();
        let multiplier = clean_schema.get_cycles_to_target((target.to_string(), "lx".to_string(), Pulse::High));
        multipliers.push(multiplier);
    } 

    let result = lcm_of(&multipliers);
    println!("Part 2 answer: {}", result);
    return Ok(());
}

// Least Common Multiple Calculation - https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm_of(steps: &[i64]) -> i64 {
    let mut iter = steps.iter();

    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    let mut ans = lcm(*first, *second);
    while let Some(x) = iter.next() {
        ans = lcm(ans, *x);
    }

    ans
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

// Greatest Common Divisor Calculation - https://en.wikipedia.org/wiki/Greatest_common_divisor
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}


/*--- Day 20: Pulse Propagation ---
With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send the command to boot up the machines and get the sand flowing again.

The machines are far apart and wired together with long cables. The cables don't connect to the machines directly, but rather to communication modules attached to the machines that perform various initialization tasks and also act as communication relays.

Modules communicate using pulses. Each pulse is either a high pulse or a low pulse. When a module sends a pulse, it sends that type of pulse to each module in its list of destination modules.

There are several different types of modules:

Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.

Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.

Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, the button module. When you push the button, a single low pulse is sent directly to the broadcaster module.

After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it again. Never push the button if modules are still processing pulses.

Pulses are always processed in the order they are sent. So, if a pulse is sent to modules a, b, and c, and then module a processes its pulse and sends more pulses, the pulses sent to modules b and c would have to be handled first.

The module configuration (your puzzle input) lists each module. The name of the module is preceded by a symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules. For example:

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
In this module configuration, the broadcaster has three destination modules named a, b, and c. Each of these modules is a flip-flop module (as indicated by the % prefix). a outputs to b which outputs to c which outputs to another module named inv. inv is a conjunction module (as indicated by the & prefix) which, because it has only one input, acts like an inverter (it sends the opposite of the pulse type it receives); it outputs to a.

By pushing the button once, the following pulses are sent:

button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a
After this sequence, the flip-flop modules all end up off, so pushing the button again repeats the same sequence.

Here's a more interesting example:

broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
This module configuration includes the broadcaster, two flip-flops (named a and b), a single-input conjunction module (inv), a multi-input conjunction module (con), and an untyped module named output (for testing purposes). The multi-input conjunction module con watches the two flip-flop modules and, if they're both on, sends a low pulse to the output module.

Here's what happens if you push the button once:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output
Both flip-flops turn on and a low pulse is sent to output! However, now that both flip-flops are on and con remembers a high pulse from each of its two inputs, pushing the button a second time does something different:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
Flip-flop a turns off! Now, con remembers a low pulse from module a, and so it sends only a high pulse to output.

Push the button a third time:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output
This time, flip-flop a turns on, then flip-flop b turns off. However, before b can turn off, the pulse sent to con is handled first, so it briefly remembers all high pulses for its inputs and sends a low pulse to output. After that, flip-flop b turns off, which causes con to update its state and send a high pulse to output.

Finally, with a on and b off, push the button a fourth time:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output
This completes the cycle: a turns off, causing con to remember only low pulses and restoring all modules to their original states.

To get the cables warmed up, the Elves have pushed the button 1000 times. How many pulses got sent as a result (including the pulses sent by the button itself)?

In the first example, the same thing happens every time the button is pushed: 8 low pulses and 4 high pulses are sent. So, after pushing the button 1000 times, 8000 low pulses and 4000 high pulses are sent. Multiplying these together gives 32000000.

In the second example, after pushing the button 1000 times, 4250 low pulses and 2750 high pulses are sent. Multiplying these together gives 11687500.

Consult your module configuration; determine the number of low pulses and high pulses that would be sent after pushing the button 1000 times, waiting for all pulses to be fully handled after each push of the button. What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?

--- Part Two ---
The final machine responsible for moving the sand down to Island Island has a module attached named rx. The machine turns on when a single low pulse is sent to rx.

Reset all modules to their default states. Waiting for all pulses to be fully handled after each button press, what is the fewest number of button presses required to deliver a single low pulse to the module named rx?*/