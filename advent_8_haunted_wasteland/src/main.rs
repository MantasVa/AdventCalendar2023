use std::{fs, collections::HashMap};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Map {
    commands: Vec<Command>,
    locations: HashMap<String, Location>
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Location {
    curr: String,
    left: String,
    right: String
}

impl Location {
    fn new(curr: String, left: String, right: String) -> Location {
        Location { curr, left, right }
    }
}

enum Command {
    Left,
    Right
}

impl Command {
    fn new(cmd: char) -> Command {
        match cmd {
            'L' => Command::Left,
            'R' => Command::Right,
            _ => panic!("Unhandled input")
        }
    }
}

fn main() -> Result<()> {
    let map = parse()?;

    let _result1 = part1(&map)?;
    let result2 = part2(&map);

    result2
}

fn parse() -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split("\r\n").collect::<Vec<_>>();

    let commands = input[0].chars().map(|c| Command::new(c)).collect::<Vec<_>>();

    let trims = ['(', ')', ' '];
    let locations = input[2..].iter()
        .map(|s| s.split_once("=").unwrap())
        .map(|(c, l)| (c.trim(), l.trim_matches(&trims[0..])))
        .map(|(c, l)| (c, l.split_once(',').unwrap()))
        .map(|(c, l)| (String::from(c), Location::new(String::from(c), String::from(l.0.trim()), String::from(l.1.trim()))))
        .collect::<HashMap<String, Location>>();

    Ok(Map { commands, locations })
}

fn part1(map: &Map) -> Result<()> {
    let mut loc = "AAA";

    let commands_count = map.commands.len();
    let mut command_idx = 0;

    let mut steps = 0;
    while loc != "ZZZ" {
        let curr = map.locations.get(loc).unwrap();

        loc = match map.commands[command_idx] {
            Command::Left => curr.left.as_str(),
            Command::Right => curr.right.as_str()
        };
        steps += 1;

        if command_idx + 1 == commands_count {
            command_idx = 0
        } else {
            command_idx += 1
        }
    }

    println!("Part 1 answer: {}", steps);
    return Ok(());
}

fn part2(map: &Map) -> Result<()> {
    let queue = map.locations.keys()
        .filter(|l| l.ends_with('A'))
        .collect::<Vec<_>>();

    let commands_count = map.commands.len();
    let mut command_idx ;

    let mut steps: Vec<i64> = Vec::new();
    for from in queue {
        let mut curr = from.as_str();
        command_idx = 0;
    
        let mut count = 0;
        while !curr.ends_with('Z') {
            curr = match map.commands[command_idx] {
                Command::Left => map.locations.get(curr).unwrap().left.as_str(),
                Command::Right => map.locations.get(curr).unwrap().right.as_str()
            };
            count += 1;
    
            if command_idx + 1 == commands_count {
                command_idx = 0
            } else {
                command_idx += 1
            }
        }

        steps.push(count);
    }

    let ans = lcm_of(&steps);

    println!("Part 2 answer: {}", ans);
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

/* --- Day 8: Haunted Wasteland ---
You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ? 

--- Part Two ---
The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

Step 0: You are at 11A and 22A.
Step 1: You choose all of the left paths, leading you to 11B and 22B.
Step 2: You choose all of the right paths, leading you to 11Z and 22C.
Step 3: You choose all of the left paths, leading you to 11B and 22Z.
Step 4: You choose all of the right paths, leading you to 11Z and 22B.
Step 5: You choose all of the left paths, leading you to 11B and 22C.
Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?*/