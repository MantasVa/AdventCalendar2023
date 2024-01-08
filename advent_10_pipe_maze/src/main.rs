use std::{fs, vec, collections::{VecDeque, HashMap}};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Map {
    start: (i32, i32),
    pipes: HashMap<(i32, i32), Pipe>,
}

struct Pipe {
    pipe_type: PipeType,
    dir_1: Direction,
    dir_2: Direction,
    coords: (i32, i32)
}

impl Pipe {
    fn neighbors(&self) -> Vec<(i32, i32)> {
        let (x, y) = self.coords;
        vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    }
}

impl Pipe {
    fn new(c: char, coords: (i32, i32)) -> Pipe {
        match c {
            '|' => Pipe { pipe_type: PipeType::Vertical, dir_1: Direction::North, dir_2: Direction::South, coords },
            '-' => Pipe { pipe_type: PipeType::Horizontal, dir_1: Direction::East, dir_2: Direction::West, coords  },
            'L' => Pipe { pipe_type: PipeType::Bend, dir_1: Direction::North, dir_2: Direction::East, coords  },
            'J' => Pipe { pipe_type: PipeType::Bend, dir_1: Direction::North, dir_2: Direction::West, coords  },
            '7' => Pipe { pipe_type: PipeType::Bend, dir_1: Direction::South, dir_2: Direction::West, coords  },
            'F' => Pipe { pipe_type: PipeType::Bend, dir_1: Direction::South, dir_2: Direction::East, coords  },
            'S' => Pipe { pipe_type: PipeType::Any, dir_1: Direction::Any, dir_2: Direction::Any, coords  },
            _ => panic!("Bad input")
        }
    }

    fn can_connect(&self, connector: &Pipe) -> bool {
        let (x1, y1) = self.coords;
        let (x2, y2) = connector.coords;
        
        if x1 == x2 + 1 && y1 == y2 &&
           (self.dir_1 == Direction::North || self.dir_2 == Direction::North || self.dir_1 == Direction::Any) &&
           (connector.dir_1 == Direction::South || connector.dir_2 == Direction::South || connector.dir_1 == Direction::Any) {
            true
        } else if x1 == x2 - 1 && y1 == y2 &&
            (self.dir_1 == Direction::South || self.dir_2 == Direction::South || self.dir_1 == Direction::Any) &&
            (connector.dir_1 == Direction::North || connector.dir_2 == Direction::North || connector.dir_1 == Direction::Any) {
            true
        } else if x1 == x2 && y1 + 1 == y2 &&
            (self.dir_1 == Direction::East || self.dir_2 == Direction::East || self.dir_1 == Direction::Any) &&
            (connector.dir_1 == Direction::West || connector.dir_2 == Direction::West || connector.dir_1 == Direction::Any) {
            true
        } else if x1 == x2 && y1 - 1 == y2 &&
            (self.dir_1 == Direction::West || self.dir_2 == Direction::West || self.dir_1 == Direction::Any) &&
            (connector.dir_1 == Direction::East || connector.dir_2 == Direction::East || connector.dir_1 == Direction::Any) {
            true
        } else {
            false
        }
    }
}

enum PipeType {
    Any,
    Vertical,
    Horizontal,
    Bend
}

#[derive(PartialEq, Eq)]
enum Direction {
    Any,
    North,
    South,
    East,
    West
}

fn main() -> Result<()> {
    let map = parse()?;

    let pipe_loop = part1(&map)?;
    let result2 = part2(&map, pipe_loop);

    result2
}

fn parse() -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;

    let mut pipes: HashMap<(i32, i32), Pipe> = HashMap::new();
    let mut start = (0, 0);
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char != '.' && !char.is_whitespace() {
                pipes.insert((x as i32, y as i32), Pipe::new(char, (x as i32, y as i32)));
            }

            if char == 'S' {
                start = (x as i32, y as i32);
            }
        }
    }

    Ok(Map { start, pipes })
}

fn part1(map: &Map) -> Result<Vec<(i32, i32)>> {
    let mut pipe_loop: Vec<(i32, i32)> = vec![map.start];

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.extend(map.pipes[&map.start].neighbors());
    let mut from = map.start;
    while let Some(to) = queue.pop_front() {
        let connector = &map.pipes[&from];
        if let Some(connectee) = &map.pipes.get(&to) {

            if connector.can_connect(&connectee) && !pipe_loop.contains(&connectee.coords) {
                pipe_loop.push(connectee.coords);
                
                from = connectee.coords;
                queue.extend(connectee.neighbors());
            }
        }
    }

    let farthest_point = (pipe_loop.len() as f32 / 2.).ceil(); 
    println!("Part 1 answer: {}", farthest_point);
    return Ok(pipe_loop);
}

fn part2(map: &Map, pipe_loop: Vec<(i32, i32)>) -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut inside = false;
    let mut count = 0;
    for (x, line) in input.lines().enumerate() {
        let mut last_dir = (Direction::Any, Direction::Any);
        for (y, _) in line.chars().enumerate() {
            if pipe_loop.contains(&(x as i32, y as i32)) {
                let pipe = &map.pipes[&(x as i32, y as i32)];

                match pipe.pipe_type {
                    PipeType::Vertical => inside = !inside,
                    PipeType::Horizontal => (),
                    PipeType::Bend if pipe.dir_1 == Direction::North && pipe.dir_2 == Direction::East => last_dir = (Direction::North, Direction::East),
                    PipeType::Bend if pipe.dir_1 == Direction::South && pipe.dir_2 == Direction::East => last_dir = (Direction::South, Direction::East),
                    PipeType::Bend if pipe.dir_1 == Direction::South && pipe.dir_2 == Direction::West && last_dir == (Direction::North, Direction::East) => inside = !inside,
                    PipeType::Bend if pipe.dir_1 == Direction::North && pipe.dir_2 == Direction::West && last_dir == (Direction::South, Direction::East) => inside = !inside,
                    _ => (),
                }
            }
            else if inside {
                count += 1;
            }
        }
    }
    
    println!("Part 2 answer: {}", count);
    return Ok(());
}


/* --- Day 10: Pipe Maze ---
You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....
If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....
In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF
In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....
You can count the distance each tile in the loop is from the starting point like this:

.....
.012.
.1.3.
.234.
.....
In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...
Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?*/