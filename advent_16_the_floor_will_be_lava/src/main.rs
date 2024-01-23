use std::{collections::HashSet, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Map {
    map: Vec<Vec<Type>>,
    rows: i32,
    cols: i32,
}

impl Map {
    fn get_score(&self, start: ((i32, i32), i32)) -> i32 {
        let mut beams = vec![start];

        let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();
        while let Some(b) = beams.pop() {
            if !visited.insert(b) {
                continue;
            }

            let x = b.0.0 as usize;
            let y = b.0.1 as usize;
            let dir = b.1 as usize;

            match self.map[x][y] {
                Type::SlashMirror => {
                    let new_dir = 3 - dir;
                    if let Some(new_pos) = self.next_pos(b.0, new_dir){
                        beams.push((new_pos, new_dir as i32))
                    }
                },
                Type::BackWardSlashMirror => {
                    let new_dir = (dir + 2) % 4;
                    if let Some(new_pos) = self.next_pos(b.0, new_dir){
                        beams.push((new_pos, new_dir as i32))
                    }
                },
                Type::VerticalSplitter => {
                    if b.1 == 2 || b.1 == 3 {
                        for new_dir in 0..=1 {
                            if let Some(new_pos) = self.next_pos(b.0, new_dir){
                                beams.push((new_pos, new_dir as i32))
                            }
                        }
                    } else {
                        if let Some(new_pos) = self.next_pos(b.0, dir){
                            beams.push((new_pos, b.1))
                        }
                    }
                },
                Type::HorizontalSplitter => {
                    if b.1 == 0 || b.1 == 1 {
                        for new_dir in 2..=3 {
                            if let Some(new_pos) = self.next_pos(b.0, new_dir){
                                beams.push((new_pos, new_dir as i32))
                            }
                        }
                    } else {
                        if let Some(new_pos) = self.next_pos(b.0, dir){
                            beams.push((new_pos, b.1))
                        }
                    }
                },
                Type::Empty => {
                    if let Some(new_pos) = self.next_pos(b.0, dir){
                        beams.push((new_pos, b.1))
                    }
                },
            }
        }

        visited.iter().map(|x| x.0).collect::<HashSet<(i32, i32)>>().len() as i32
    }

    fn next_pos(&self, b: (i32, i32), dir: usize) -> Option<(i32, i32)> {
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1) ]; // up, down, left, right

        let new_pos = (b.0 + dirs[dir].0, b.1 + dirs[dir].1);
        if new_pos.0 < 0 || new_pos.0 >= self.rows ||
           new_pos.1 < 0 || new_pos.1 >= self.cols {
            None
           } else {
            Some(new_pos)
           }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Type {
    SlashMirror,
    BackWardSlashMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Empty
}

impl Type {
    fn new(c: char) -> Type {
        match c {
            '/' => Type::SlashMirror,
            '\\' => Type::BackWardSlashMirror,
            '|' => Type::VerticalSplitter,
            '-' => Type::HorizontalSplitter,
            '.' => Type::Empty,
            _ => panic!("Bad input")
        }
    }
}

fn main() -> Result<()> {
    let map = parse()?;

    _ = part1(&map)?;
    _ = part2(&map)?;

    return Ok(());
}

fn parse() -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;

    let mut map: Vec<Vec<Type>> = Vec::new();
    for (x,line) in input.lines().enumerate() {
        map.push(Vec::new());
        for char in line.chars() {
            map[x].push(Type::new(char));
        }
    }

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    Ok(Map { map, rows, cols })
}

fn part1(map: &Map) -> Result<()> {
    let score = map.get_score(((0, 0), 3));
    println!("Part 1 answer: {}", score);
    return Ok(());
}

fn part2(map: &Map) -> Result<()> {
    let mut max_score = 0;

    for y in 0..map.cols {
        let down_score = map.get_score(((0, y), 1));

        if down_score > max_score {
            max_score = down_score
        }

        let up_score = map.get_score(((map.rows - 1, y), 0));
        if up_score > max_score {
            max_score = up_score
        }
    }

    for x in 0..map.rows {
        let right_score = map.get_score(((x, 0), 3));

        if right_score > max_score {
            max_score = right_score
        }

        let left_score = map.get_score(((x, map.cols - 1), 2));
        if left_score > max_score {
            max_score = left_score
        }
    }

    println!("Part 2 answer: {}", max_score);
    return Ok(());
}

/* --- Day 16: The Floor Will Be Lava ---
With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

If the beam encounters empty space (.), it continues in the same direction.
If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..
Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?

--- Part Two ---
As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..
Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration? */