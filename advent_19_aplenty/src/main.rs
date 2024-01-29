use std::{collections::{HashMap, VecDeque}, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Command(String, Option<Condition>);

#[derive(Clone)]
struct Condition(String, Sign, i64);

struct Part(i64, i64, i64, i64);

impl Part {
    fn sum(&self) -> i64 {
        self.0 + self.1 + self.2 + self.3
    }
}

struct Workflow {
    commands: HashMap<String, Vec<Command>>,
    parts: Vec<Part>
}

impl Workflow {
    fn get_combinations(&self, coms: Vec<Command>, mut ranges: Vec<(usize, usize)>) -> usize {
        let mut possibilities = 0usize;
    
        let mut queue = VecDeque::new();
        queue.extend(coms);
        
        while let Some(c) = queue.pop_front() {
            let mut deeper = ranges.clone();
            match c.1 {
                Some(Condition(feature, Sign::More, threshold)) => {
                    let idx = Workflow::get_range_index(feature.as_str());
                    let threshold = threshold as usize;

                    if deeper[idx].1 > threshold {
                        deeper[idx] = (deeper[idx].0.max(threshold + 1), deeper[idx].1);
                        possibilities += match c.0.as_str() {
                            "A" => self.get_range_poss(&deeper),
                            "R" => 0,
                            to => self.get_combinations(self.commands[to].clone(), deeper)
                        }
                    }

                    if ranges[idx].0 < threshold {
                        ranges[idx] = (ranges[idx].0, threshold);
                    } else {
                        break;
                    }
                },
                Some(Condition(feature, Sign::Less, threshold)) => {
                    let idx = Workflow::get_range_index(feature.as_str());
                    let threshold = threshold as usize;
                    
                    if deeper[idx].0 < threshold {
                        deeper[idx] = (deeper[idx].0, deeper[idx].1.min(threshold - 1));
                        possibilities += match c.0.as_str() {
                            "A" => self.get_range_poss(&deeper),
                            "R" => 0,
                            to => self.get_combinations(self.commands[to].clone(), deeper)
                        }
                    } 

                    if ranges[idx].1 > threshold {
                        ranges[idx] = (threshold, ranges[idx].1);
                    } else {
                        break;
                    }
                },
                None => {
                    possibilities += match c.0.as_str() {
                        "A" => self.get_range_poss(&ranges),
                        "R" => 0,
                        to => self.get_combinations(self.commands[to].clone(), ranges.clone())
                    }
                }
            }
        }

        possibilities
    }

    fn get_range_poss(&self, ranges: &Vec<(usize, usize)>) -> usize {
        ranges.iter().map(|(start, end)| (*start..*end).len() + 1).product()
    }

    fn get_range_index(feature: &str) -> usize {
        match feature {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => panic!("Bad input")
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Sign {
    More,
    Less
}

impl Sign {
    fn new(c: char)  -> Sign {
        match c {
            '>' => Sign::More,
            '<' => Sign::Less,
            _ => panic!("Bad input")
        }
    }
}

fn main() -> Result<()> {
    let workflow = parse()?;

    part1(&workflow)?;
    part2(&workflow)?;

    return Ok(());
}

fn parse() -> Result<Workflow> {
    let input = fs::read_to_string("input.txt")?;

    let mut commands = HashMap::new();
    let (comms, part_feat) = input.split_once("\r\n\r\n").unwrap();
    for com in comms.lines() {
        let (name, rest) = com.split_once('{').unwrap();

        let mut flow_commands = Vec::new();
        for command in rest.trim_end_matches('}').split(',') {

            if command.contains('>') || command.contains('<'){
                let feature = command.chars().nth(0).unwrap().to_string();
                let sign = Sign::new(command.chars().nth(1).unwrap());
                let threshold = command[2..command.find(|x | x == ':').unwrap()].to_string();
                let to = command[command.find(|x | x == ':').unwrap() + 1..].to_string();
            
                let command = Command(to, Some(Condition(feature, sign, threshold.parse().unwrap())));
                flow_commands.push(command);
            } else {
                let variable = command.to_string();
                flow_commands.push(Command(variable, None));

            }
        }

        commands.insert(name.to_string(), flow_commands);
    }

    let mut parts = Vec::new();
    for part_features in part_feat.lines() {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for feature in part_features.split(',') {
            let (unparsed_feat, unp_ranking) = feature.split_once('=').unwrap();
            let feature = unparsed_feat.trim_matches(|c| c == '{' || c == '}').to_string();
            let ranking = unp_ranking.trim_matches(|c| c == '{' || c == '}').to_string();

            match feature.as_str() {
                "x" => x = ranking.trim().parse().unwrap(),
                "m" => m = ranking.trim().parse().unwrap(),
                "a" => a = ranking.trim().parse().unwrap(),
                "s" => s = ranking.trim().parse().unwrap(),
                _ => panic!("Bad input")
            }
        }

        parts.push(Part(x, m, a, s))
    }

    Ok(Workflow { commands, parts})
}

fn part1(workflow: &Workflow) -> Result<()> {
    const START: &str = "in";
    let mut result = 0i64;

    for part in &workflow.parts {
        let mut queue = VecDeque::new();
        queue.push_back(workflow.commands.get(START).unwrap());

        while let Some(c) = queue.pop_front() {
            for command in c {
                if command.1.is_some() {
                    let condition = command.1.clone().unwrap();
                    let (mark, sign, threshold) = (condition.0.as_str(), condition.1, condition.2);

                    if  (mark == "x" && sign == Sign::More && part.0 > threshold) ||
                        (mark == "x" && sign == Sign::Less && part.0 < threshold) ||
                        (mark == "m" && sign == Sign::More && part.1 > threshold) ||
                        (mark == "m" && sign == Sign::Less && part.1 < threshold) ||
                        (mark == "a" && sign == Sign::More && part.2 > threshold) ||
                        (mark == "a" && sign == Sign::Less && part.2 < threshold) ||
                        (mark == "s" && sign == Sign::More && part.3 > threshold) ||
                        (mark == "s" && sign == Sign::Less && part.3 < threshold) {
                            match command.0.as_str() {
                                "A" => result += part.sum(),
                                "R" => (),
                                to => queue.push_back(workflow.commands.get(to).unwrap())
                            }
                            break;
                        } else {
                            continue;
                        }
                } else {
                    match command.0.as_str() {
                        "A" => result += part.sum(),
                        "R" => (),
                        to => queue.push_back(workflow.commands.get(to).unwrap())
                    }
                    break;
                }
            }
        }
    }

    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(workflow: &Workflow) -> Result<()> {
    const START: &str = "in";
    let result = workflow.get_combinations(workflow.commands[START].clone(), vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)]);
    println!("Part 2 answer: {}", result);
    return Ok(());
}

/*

--- Day 19: Aplenty ---
The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someone stole from Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.

As you reach the bottom of the relentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have a system.

To start, each part is rated in each of four categories:

x: Extremely cool looking
m: Musical (it makes a noise when you hit it)
a: Aerodynamic
s: Shiny
Then, each part is sent through a series of workflows that will ultimately accept or reject the part. Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

Rule "x>10:one": If the part's x is more than 10, send the part to the workflow named one.
Rule "m<20:two": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is immediately rejected (R).
Rule "A": Otherwise, because no other rules matched the part, the part is immediately accepted (A).
If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.

The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:

px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow named in. In this example, the five listed parts go through the following workflows:

{x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
{x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
{x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
{x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
{x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A
Ultimately, three parts are accepted. Adding up the x, m, a, and s rating for each of the accepted parts gives 7540 for the part with x=787, 4623 for the part with x=2036, and 6951 for the part with x=2127. Adding all of the ratings for all of the accepted parts gives the sum total of 19114.

Sort through all of the parts you've been given; what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted?

--- Part Two ---
Even with your help, the sorting process still isn't fast enough.

One of the Elves comes up with a new plan: rather than sort parts individually through all of these workflows, maybe you can figure out in advance which combinations of ratings will be accepted or rejected.

Each of the four ratings (x, m, a, s) can have an integer value ranging from a minimum of 1 to a maximum of 4000. Of all possible distinct combinations of ratings, your job is to figure out which ones will be accepted.

In the above example, there are 167409079868000 distinct combinations of ratings that will be accepted.

Consider only your list of workflows; the list of part ratings that the Elves wanted you to sort is no longer relevant. How many distinct combinations of ratings will be accepted by the Elves' workflows?*/