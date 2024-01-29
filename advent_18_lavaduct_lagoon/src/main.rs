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

/* --- Day 18: Lavaduct Lagoon ---
Thanks to your efforts, the machine parts factory is one of the first factories up and running since the lavafall came back. However, to catch up with the large backlog of parts requests, the factory will also need a large supply of lava for a while; the Elves have already started creating a large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the dig plan (your puzzle input). For example:

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters up (U), down (D), left (L), or right (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with the color that the edge of the trench should be painted as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the following loop of trench (#) having been dug out from otherwise ground-level terrain (.):

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
At this point, the trench could contain 38 cubic meters of lava. However, this is just the edge of the lagoon; the next step is to dig out the interior so that it is one meter deep as well:

#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######
Now, the lagoon can contain a much more respectable 62 cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, how many cubic meters of lava could it hold?

--- Part Two ---
The Elves were right to be concerned; the planned lagoon would be much too small.

After a few minutes, someone realizes what happened; someone swapped the color and instruction parameters when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

Each hexadecimal code is six hexadecimal digits long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.

So, in the above example, the hexadecimal codes can be converted into the true instructions:

#70c710 = R 461937
#0dc571 = D 56407
#5713f0 = R 356671
#d2c081 = D 863240
#59c680 = R 367720
#411b91 = D 266681
#8ceee2 = L 577262
#caa173 = U 829975
#1b58a2 = L 112010
#caa171 = D 829975
#7807d2 = L 491645
#a77fa3 = U 686074
#015232 = L 5411
#7a21e3 = U 500254
Digging out this loop and its interior produces a lagoon that can hold an impressive 952408144115 cubic meters of lava.

Convert the hexadecimal color codes into the correct instructions; if the Elves follow this new dig plan, how many cubic meters of lava could the lagoon hold? */