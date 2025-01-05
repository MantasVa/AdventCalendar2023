use std::{collections::{HashMap, HashSet, VecDeque}, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Map {
    tiles: HashMap<(i64, i64), Type>,
    start: (i64, i64),
    end: (i64, i64),
}

impl Map {
    const NEIGHBORS: [(i64, i64, Dir); 4] = [(0, 1, Dir::East), (0, -1, Dir::West), (1, 0, Dir::South), (-1, 0, Dir::North), ];

    fn explore(&self, curr: (i64, i64), mut visited: HashSet<(i64, i64)>, count: i64) -> ((i64, i64), i64) {
        let next = self.get_neighbors(curr, &visited);

        if curr == self.end || next.len() == 0 {
            return (curr, count);
        }

        visited.insert(curr);
        let new_count = count + 1;
        let mut max = count;
        let mut new_p = (0, 0);
        for (pos, _) in next {
            let (new_pos, new_count) = self.explore(pos, visited.clone(), new_count);

            if new_count > max && new_pos == self.end {
                max = new_count;
                new_p = new_pos;
            }
        }

        (new_p, max)
    }

    fn get_neighbors(&self, curr: (i64, i64), visited: &HashSet<(i64, i64)>) -> Vec<((i64, i64), Dir)> {
        Self::NEIGHBORS.iter().filter_map(|pos|{
            let new_pos = (curr.0 + pos.0, curr.1 + pos.1);
            if self.tiles.contains_key(&new_pos) && !visited.contains(&new_pos) {
             return match self.tiles[&new_pos] {
                 Type::Path => Some((new_pos, pos.2)),
                 Type::Hill(dir) if dir == pos.2 => Some((new_pos, pos.2)),
                 _ => None
             }
            }
            None
         }).collect::<Vec<_>>()
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
enum Type {
    Path,
    Hill(Dir)
}

impl Type {
    fn new (c: char)-> Option<Type> {
        match c {
            '.' => Some(Type::Path),
            '>' => Some(Type::Hill(Dir::East)),
            '<' => Some(Type::Hill(Dir::West)),
            'v' => Some(Type::Hill(Dir::South)),
            '^' => Some(Type::Hill(Dir::North)),
            _ => None
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Copy)]
enum Dir {
    East,
    West,
    South,
    North
}

fn main() -> Result<()> {
    let map_hills = parse(true)?;
    part1(&map_hills)?;

    let map = parse(false)?;
    part2(&map)?;

    return Ok(());
}

fn parse(parse_hills: bool) -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;

    let mut start = (0, 0);
    let mut tiles = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, tile) in line.chars().enumerate() {
            if let Some(typ) = Type::new(tile) {
                if parse_hills {
                    tiles.insert((x as i64, y as i64), typ);
                } else {
                    tiles.insert((x as i64, y as i64), Type::Path);
                }

                if x == 0 {
                    start = (x as i64, y as i64);
                }
            }
        }
    }

    let end = tiles.iter().max_by(|a, b| a.0.0.cmp(&b.0.0)).unwrap().0.clone();

    Ok(Map { tiles, start, end })
}

fn part1(map: &Map) -> Result<()> {
    let (_, count) = map.explore(map.start, HashSet::new(), 0);

    println!("Part 1 answer: {}", count);
    return Ok(());
}

fn part2(map: &Map) -> Result<()> {
    let mut max = 0;
    let mut queue = VecDeque::new();
    queue.push_back((map.start, HashSet::<(i64, i64)>::new(), 0));


    while let Some((pos, mut visited, count)) = queue.pop_front() {
        let neighbors = map.get_neighbors(pos, &visited);
        visited.insert(pos);

        if pos == map.end && count > max {
            max = count;
            println!("New max is {max}");
        }

        for (new_pos, _) in neighbors {
            queue.push_back((new_pos, visited.clone(), count + 1));
        }
    }

    println!("Part 2 answer: {}", max);
    return Ok(());
}

/*--- Day 23: A Long Walk ---
The Elves resume water filtering operations! Clean water starts flowing over the edge of Island Island.

They offer to help you go over the edge of Island Island, too! Just hold on tight to one end of this impossibly long rope and they'll lower you down a safe distance from the massive waterfall you just created.

As you finally reach Snow Island, you see that the water isn't really reaching the ground: it's being absorbed by the air itself. It looks like you'll finally have a little downtime while the moisture builds up to snow-producing levels. Snow Island is pretty scenic, even without any snow; why not take a walk?

There's a map of nearby hiking trails (your puzzle input) that indicates paths (.), forest (#), and steep slopes (^, >, v, and <).

For example:

#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
You're currently on the single path tile in the top row; your goal is to reach the single path tile in the bottom row. Because of all the mist from the waterfall, the slopes are probably quite icy; if you step onto a slope tile, your next step must be downhill (in the direction the arrow is pointing). To make sure you have the most scenic hike possible, never step onto the same tile twice. What is the longest hike you can take?

In the example above, the longest hike you can take is marked with O, and your starting position is marked S:

#S#####################
#OOOOOOO#########...###
#######O#########.#.###
###OOOOO#OOO>.###.#.###
###O#####O#O#.###.#.###
###OOOOO#O#O#.....#...#
###v###O#O#O#########.#
###...#O#O#OOOOOOO#...#
#####.#O#O#######O#.###
#.....#O#O#OOOOOOO#...#
#.#####O#O#O#########v#
#.#...#OOO#OOO###OOOOO#
#.#.#v#######O###O###O#
#...#.>.#...>OOO#O###O#
#####v#.#.###v#O#O###O#
#.....#...#...#O#O#OOO#
#.#########.###O#O#O###
#...###...#...#OOO#O###
###.###.#.###v#####O###
#...#...#.#.>.>.#.>O###
#.###.###.#.###.#.#O###
#.....###...###...#OOO#
#####################O#
This hike contains 94 steps. (The other possible hikes you could have taken were 90, 86, 82, 82, and 74 steps long.)

Find the longest hike you can take through the hiking trails listed on your map. How many steps long is the longest hike?*/