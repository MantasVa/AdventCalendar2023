use std::{fs, collections::{HashMap, BTreeSet}};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Platform {
    rocks: HashMap<(i64, i64), Rock>,
    rows: i64,
    cols: i64,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Rock {
    Moving,
    Standing
}

impl Rock {
    fn to_rock(c: char) -> Rock {
        match c {
            'O' => Rock::Moving,
            '#' => Rock::Standing,
            _ => panic!("Bad input")
        }
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn all() -> Vec::<Direction> {
        vec![Direction::North, Direction::West, Direction::South, Direction::East]
    }
}

fn main() -> Result<()> {
    let platform = parse()?;
    _ = part1(&platform)?;
    _ = part2(&platform)?;

    return Ok(());
}

fn parse() -> Result<Platform> {
    let input = fs::read_to_string("input.txt")?;

    let mut rocks: HashMap<(i64, i64), Rock> = HashMap::new();
    let mut rows = 1;
    let mut cols = 1;
    
    for (x, line) in input.lines().rev().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c != '.' {
                _ = rocks.insert((x as i64 + 1, y as i64 + 1), Rock::to_rock(c));
            }
            cols = y as i64 + 1
        }
        rows = x as i64 + 1
    }

    Ok(Platform { rocks, cols, rows })
}

fn part1(platform: &Platform) -> Result<()> {
    let mut result = 0;
    for y in 1..platform.cols + 1 {
        let mut moved: HashMap<(i64, i64), Rock> = HashMap::new();

        for x in (1..platform.rows + 1).rev() {
            if let Some(r) = platform.rocks.get(&(x, y)) {
                match r {
                    Rock::Moving => {
                        let rock_above = moved.iter()
                            .filter(|(coord,  _)| coord.1 == y && coord.0 > x)
                            .map(|(coord,  _)| *coord)
                            .min_by(|x, y| x.0.cmp(&y.0));

                        if rock_above.is_some() {
                            _ = moved.insert((rock_above.unwrap().0 - 1, y), *r);
                        } else {
                            _ = moved.insert((platform.rows, y), *r);
                        }
                    },
                    Rock::Standing => _ = moved.insert((x, y), *r)
                }
            }
        }

        result += moved.iter().filter_map(|(coord, rock)| if *rock == Rock::Moving { Some(coord.0) } else { None }).sum::<i64>();
    }
    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(platform: &Platform) -> Result<()> {
    let mut cycle = 0;

    let mut tilted_plat = platform.clone();

    let mut loops = HashMap::new();
    let key = tilted_plat.rocks.keys().copied().collect::<BTreeSet<_>>();
    loops.insert(key, cycle);

    let (start, end) = loop {
        cycle += 1;

        for dir in Direction::all() {
            tilted_plat = tilt(&tilted_plat, dir);
        }

        let key = tilted_plat.rocks.keys().copied().collect::<BTreeSet<_>>();
        if let Some(val) = loops.insert(key, cycle) {
            break (val, cycle);
        }
    };

    let diff = end - start;
    let remaining = 1000000000 - start;
    let phase = remaining % diff;

    for _ in 0..phase {
        for dir in Direction::all() {
            tilted_plat = tilt(&tilted_plat, dir);
        }
    }

    let result = tilted_plat.rocks.iter().filter_map(|(coord, rock)| if *rock == Rock::Moving { Some(coord.0) } else { None }).sum::<i64>();
    println!("Part 2 answer: {}", result);
    return Ok(());
}

fn tilt(platform: &Platform, dir: Direction) -> Platform {
    let mut moved_rocks: HashMap<(i64, i64), Rock> = HashMap::new();

    let y_range = match dir {
        Direction::East => (1..platform.cols + 1).rev().collect::<Vec<_>>(),
        _ => (1..platform.cols + 1).collect::<Vec<_>>(),
    };

    let x_range = match dir {
        Direction::North => (1..platform.rows + 1).rev().collect::<Vec<_>>(),
        _ => (1..platform.rows + 1).collect::<Vec<_>>(),
    };

    for y in y_range {
        for x in &x_range {
            let x = *x;
            if let Some(r) = platform.rocks.get(&(x, y)) {
                match r {
                    Rock::Moving => {
                        let neighb_rocks = moved_rocks.iter()
                            .filter(|(coord,  _)| { 
                                match dir {
                                    Direction::North => coord.1 == y && coord.0 > x,
                                    Direction::West => coord.1 < y && coord.0 == x,
                                    Direction::South => coord.1 == y && coord.0 < x,
                                    Direction::East => coord.1 > y && coord.0 == x,
                                }
                             })
                            .map(|(coord,  _)| *coord);
                        
                        let neighb_rock = match dir {
                            Direction::North => neighb_rocks.min_by(|x, y| x.0.cmp(&y.0)),
                            Direction::West => neighb_rocks.max_by(|x, y| x.1.cmp(&y.1)),
                            Direction::South => neighb_rocks.max_by(|x, y| x.0.cmp(&y.0)),
                            Direction::East => neighb_rocks.min_by(|x, y| x.1.cmp(&y.1)),
                        };

                        if neighb_rock.is_some() {
                            match dir {
                                Direction::North => _ = _ = moved_rocks.insert((neighb_rock.unwrap().0 - 1, y), *r),
                                Direction::West => _ = moved_rocks.insert((x, neighb_rock.unwrap().1 + 1), *r),
                                Direction::South => _ = moved_rocks.insert((neighb_rock.unwrap().0 + 1, y), *r),
                                Direction::East => _ = _ = moved_rocks.insert((x, neighb_rock.unwrap().1 - 1), *r),
                            }
                        } else {
                            match dir {
                                Direction::North => _ = moved_rocks.insert((platform.rows, y), *r),
                                Direction::West => _ = moved_rocks.insert((x, 1), *r),
                                Direction::South => _ = moved_rocks.insert((1, y), *r),
                                Direction::East => _ = moved_rocks.insert((x, platform.cols), *r),
                            }
                        }
                    },
                    Rock::Standing => _ = moved_rocks.insert((x, y), *r)
                }
            }
        }
    }

    Platform { rocks: moved_rocks, rows: platform.rows, cols: platform.cols }
}

/* --- Day 14: Parabolic Reflector Dish ---
You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is meant to focus light, all it's doing right now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go where it's pointing and use the light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1
The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams? 

--- Part Two ---
The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled "spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.

Here's what happens in the example above after each of the first few cycles:

After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load on the north support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north support beams?*/