use std::{collections::{HashSet, VecDeque}, fs::{self, Permissions}, hash::Hash};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// Direction, Steps, Color Code
struct Command(Direction, i64, String);

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn new(c: &str) -> Direction {
        match c {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Bad direction")
        }
    }

    fn new_from_i32(c: i32) -> Direction {
        match c {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Bad direction")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i64, i64);

fn main() -> Result<()> {
    let commands = parse()?;

    part1(&commands)?;
    part2(&commands)?;

    return Ok(());
}

fn parse() -> Result<Vec<Command>> {
    let input = fs::read_to_string("input.txt")?;

    let commands = input.lines().map(|line| {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        Command(Direction::new(splits[0]), splits[1].parse().unwrap(), splits[2].trim_matches(|c| c == '(' || c == ')').to_string())
    }).collect::<Vec<Command>>();

    Ok(commands)
}

fn part1(commands: &Vec<Command>) -> Result<()> {
    let score = get_score(commands);
    println!("Part 1 answer: {}", score);
    return Ok(());
}

fn part2(commands: &Vec<Command>) -> Result<()> {
    let mut prev = Coord(0, 0);

    let mut total = 0i64;
    let mut perimeter = 0i64;
    for Command(_, _, com) in commands {
        let dir = Direction::new_from_i32(com.chars().last().unwrap().to_string().parse::<i32>().unwrap());
        let steps = i64::from_str_radix(&com[1..6], 16).unwrap();

        let next = match dir {
            Direction::Up => Coord(prev.0 - steps, prev.1),
            Direction::Down => Coord(prev.0 + steps, prev.1),
            Direction::Left => Coord(prev.0, prev.1 - steps),
            Direction::Right => Coord(prev.0, prev.1 + steps),
        };

        perimeter += steps;
        total += (prev.1 * next.0) - (prev.0 * next.1);
        prev = next;
    }

    println!("Part 2 answer: {}", (total + perimeter) / 2 + 1);
    return Ok(());
}

fn get_score(commands: &Vec<Command>) -> usize {
    let mut lagoon: HashSet<Coord> = HashSet::new();

    let mut curr = Coord(0, 0);
    _ = lagoon.insert(curr);
    for comm in commands {
        for _ in 0..comm.1 {
            match comm.0 {
                Direction::Up => curr = Coord(curr.0 - 1, curr.1),
                Direction::Down => curr = Coord(curr.0 + 1, curr.1),
                Direction::Left => curr = Coord(curr.0, curr.1 - 1),
                Direction::Right => curr = Coord(curr.0, curr.1 + 1),
            }
            _ = lagoon.insert(curr);
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(Coord(1,1));
    while let Some(c) = queue.pop_front() {
        let up_coord = Coord(c.0 - 1, c.1);
        if lagoon.insert(up_coord) {
            queue.push_back(up_coord);
        }

        let down_coord = Coord(c.0 + 1, c.1);
        if lagoon.insert(down_coord) {
            queue.push_back(down_coord);
        }

        let left_coord = Coord(c.0, c.1 - 1);
        if lagoon.insert(left_coord) {
            queue.push_back(left_coord);
        }

        let right_coord = Coord(c.0, c.1 + 1);
        if lagoon.insert(right_coord) {
            queue.push_back(right_coord);
        }
    }

    lagoon.len()
}
