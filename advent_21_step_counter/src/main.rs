use std::{collections::{HashMap, HashSet, VecDeque}, fs, vec};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Schema {
    map: HashMap<(isize, isize), Type>,
    start: (isize, isize),
    height: isize,
    width: isize,
}

#[derive(Clone, PartialEq, Eq)]
enum Type {
    Plot,
    Rock,
    Start
}

impl Type {
    fn new(c: char) -> Type {
        match c {
            '.' => Type::Plot,
            '#' => Type::Rock,
            'S' => Type::Start,
            _ => panic!("Bad input")
        }
    }
}

fn main() -> Result<()> {
    let schema = parse()?;

    part1(&schema, 64)?;
    part2(&schema, 26501365)?;

    return Ok(());
}

fn parse() -> Result<Schema> {
    let input = fs::read_to_string("input.txt")?;

    let map = input.lines().enumerate()
        .flat_map(|(x, line)| line.chars().enumerate()
        .map(move |(y, c)| ((x as isize, y as isize), Type::new(c)))).collect::<HashMap<_, _>>();

    let start = map.iter().find(|(_, typ)| **typ == Type::Start).unwrap().0;
    let height = map.iter().max_by(|a, b| a.0.1.cmp(&b.0.1)).unwrap().0.1 + 1;
    let width = map.iter().max_by(|a, b| a.0.0.cmp(&b.0.0)).unwrap().0.0 + 1;

    Ok(Schema { map: map.clone(), start: *start, height, width })
}

fn part1(schema: &Schema, steps: usize) -> Result<()> {
    let neighbours = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visited = vec![schema.start];

    for _ in 0..steps {
        let mut next_visited = HashSet::new();
        for v in visited {
            let to_step = neighbours.iter().map(|(x1, y1)| (v.0 + x1, v.1 + y1))
            .filter(|pos| schema.map.get(&pos).is_some() && *schema.map.get(&pos).unwrap() != Type::Rock);

            for step in to_step {
                _ = next_visited.insert(step);
            }
        }

        visited = next_visited.iter().copied().collect();
    }
    
    println!("Part 1 answer: {}", visited.len());
    return Ok(());
}

fn part2(schema: &Schema, steps: isize) -> Result<()> {
    let mut visited = HashSet::new();
    let mut next_visited = HashSet::new();
    let mut prev_start = 0;
    let mut start = 0;

    let remainder = steps % schema.height; // we know this is 65

    visited.insert(schema.start);
    let mut values = Vec::new();

    let mut loop_count = 0;
    while values.len() < 3 {
        loop_count += 1;

        next_visited.clear();
        for v in &visited {
            for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let pos = (v.0 + dir.0, v.1 + dir.1);
                let lookup = (
                    (v.0 + dir.0).rem_euclid(schema.height),
                    (v.1 + dir.1).rem_euclid(schema.width),
                );
                if schema.map.contains_key(&lookup) && schema.map[&lookup] != Type::Rock {
                    next_visited.insert(pos);
                }
            }
        }
        if loop_count >= remainder && (loop_count - remainder) % schema.height == 0 {
            let delta = next_visited.len() as isize - start;
            let step = [next_visited.len() as isize, delta, delta - prev_start];

            values.push(step[values.len()]);

            start = next_visited.len() as isize;
            prev_start = delta;
        }

        std::mem::swap(&mut visited, &mut next_visited);
    }
    let a = values[2] / 2;
    let b = values[1] - 3 * a;
    let c = values[0] - a - b;

    let n = 1 + steps / schema.height;

    println!("Part 2 answer: {}", a * n * n + b * n + c);
    return Ok(());
}

/*--- Day 21: Step Counter ---
You manage to catch the airship right as it's dropping someone else off on their all-expenses-paid trip to Desert Island! It even helpfully drops you off near the gardener and his massive farm.

"You got the sand flowing again! Great work! Now we just need to wait until we have enough sand to filter the water for Snow Island and we'll have snow again in no time."

While you wait, one of the Elves that works with the gardener heard how good you are at solving problems and would like your help. He needs to get his steps in for the day, and so he'd like to know which garden plots he can reach with exactly his remaining 64 steps.

He gives you an up-to-date map (your puzzle input) of his starting position (S), garden plots (.), and rocks (#). For example:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
The Elf starts at the starting position (S) which also counts as a garden plot. Then, he can take one step north, south, east, or west, but only onto tiles that are garden plots. This would allow him to reach any of the tiles marked O:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
Then, he takes a second step. Since at this point he could be at either tile marked O, his second step would allow him to reach any garden plot that is one step north, south, east, or west of any tile that he could have reached after the first step:

...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........
After two steps, he could be at any of the tiles marked O above, including the starting position (either by going north-then-south or by going west-then-east).

A single third step leads to even more possibilities:

...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........
He will continue like this until his steps for the day have been exhausted. After a total of 6 steps, he could reach any of the garden plots marked O:

...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........
In this example, if the Elf's goal was to get exactly 6 more steps today, he could use them to reach any of 16 garden plots.

However, the Elf actually needs to get 64 steps today, and the map he's handed you is much larger than the example map.

Starting from the garden plot marked S on your map, how many garden plots could the Elf reach in exactly 64 steps?*/