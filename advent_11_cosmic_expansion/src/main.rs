use std::fs;
use itertools::Itertools;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Universe {
    galaxies: Vec<(i64, i64)>
}

fn main() -> Result<()> {
    let universe_pt1 = parse(1)?;
    _ = part1(&universe_pt1)?;

    let universe_pt2 = parse(999999)?;
    _ = part2(&universe_pt2);

    return Ok(());
}

fn parse(modifier: i64) -> Result<Universe> {
    let input = fs::read_to_string("input.txt")?;

    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    let mut expander_row = 0;

    for (x, line) in input.lines().enumerate() {
        if !line.contains('#') {
            expander_row += modifier;
        }

        let mut expander_col = 0;
        for (y, char) in line.chars().enumerate() {
            if !input.lines().clone().map(|n| n.chars().nth(y).unwrap()).any(|c| c == '#') {
                expander_col += modifier;
            }

            if char == '#' {
                galaxies.push((x as i64 + expander_row, y as i64 + expander_col));
            }
        }
    }

    Ok(Universe { galaxies })
}

fn part1(universe: &Universe) -> Result<()> {
    let tot_dist = universe.galaxies.iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b))
        .unique()
        .map(|(from, to)| (from, to, get_distance((from.0, from.1), (to.0, to.1))))
        .collect::<Vec<_>>();


    println!("Part 1 answer: {}", tot_dist.iter().map(|x| x.2).sum::<i64>());
    return Ok(());
}

fn part2(universe: &Universe) -> Result<()> {
    let tot_dist = universe.galaxies.iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b))
        .unique()
        .map(|(from, to)| (from, to, get_distance((from.0, from.1), (to.0, to.1))))
        .collect::<Vec<_>>();


    println!("Part 2 answer: {}", tot_dist.iter().map(|x| x.2).sum::<i64>());
    return Ok(());
}

fn get_distance(from: (i64, i64), to: (i64, i64)) -> i64 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs() 
}

/* --- Day 11: Cosmic Expansion ---
You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^
These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......
In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......
This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

Between galaxy 1 and galaxy 7: 15
Between galaxy 3 and galaxy 6: 17
Between galaxy 8 and galaxy 9: 5
In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?*/