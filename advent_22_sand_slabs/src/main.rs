use std::{cmp, collections::{BTreeSet, HashMap, HashSet }, fs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Map {
    sand_slabs: HashMap<usize, Coord>,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Coord { x1: isize, y1: isize, z1: isize, x2: isize, y2: isize, z2: isize}

impl Coord {
    fn with_z(&self, z1: isize, z2: isize) -> Coord {
        Coord { x1: self.x1, y1: self.y1, z1, x2: self.x2, y2: self.y2, z2 }
    }

    fn collides_xy(&self, other: &Self) -> bool {
        self.x1 <= other.x2 && other.x1 <= self.x2 &&
        self.y1 <= other.y2 && other.y1 <= self.y2
    }
}

fn main() -> Result<()> {
    let map = parse()?;

    part1(&map)?;
    part2(&map)?;

    return Ok(());
}

fn parse() -> Result<Map> {
    let input = fs::read_to_string("input.txt")?;
    let mut id = 0;

    let mut sand_slabs = input.lines().map(|line| {
        let (start, end) = line.split_once('~').unwrap();

        let start = start.split(',').collect::<Vec<_>>();
        let x1 = start[0].parse().unwrap();
        let y1 = start[1].parse().unwrap();
        let z1 = start[2].parse().unwrap();

        let end = end.split(',').collect::<Vec<_>>();
        let x2 = end[0].parse().unwrap();
        let y2 = end[1].parse().unwrap();
        let z2 = end[2].parse().unwrap();
        id += 1;

        (id, Coord { x1, y1, z1, x2, y2, z2 })
    }).collect::<HashMap<_, _>>();

    Ok(Map { sand_slabs })
}

fn part1(map: &Map) -> Result<()> {
    
    // let mut slabs = Vec::<Coord>::new();
    // for slab in map.sand_slabs.iter() {
    //     let top_slab = slabs.iter().filter(|sl| slab.collides_xy(sl))
    //     .max_by(|a, b| a.z2.cmp(&b.z2));

    //     if let Some(top_sl) = top_slab {
    //         let z_ground = top_sl.z2 + 1;
    //         let dist = slab.z1 - z_ground;

    //         let z1 = slab.z1 - dist;
    //         let z2 = slab.z2 - dist;

    //         slabs.push(slab.with_z(z1, z2))
    //     } else {
    //         if slab.z1 == 1 {
    //             slabs.push(slab.clone())
    //         } else {
    //             let dist = slab.z1 - 1;

    //             let z1 = slab.z1 - dist;
    //             let z2 = slab.z2 - dist;

    //             slabs.push(slab.with_z(z1, z2))
    //         }
    //     }
    // }

    // let mut counter = 0;
    // let max_z = slabs.iter().max_by(|a, b| a.z2.cmp(&b.z2)).unwrap().z2;
    // for slab in &slabs {
    //     let supp_z1 = &slabs.iter().find(|sl| (sl.z1 == slab.z1 || sl.z2 == slab.z1) && sl.id != slab.id);
    //     let supp_z2 = &slabs.iter().find(|sl| (sl.z1 == slab.z2 || sl.z2 == slab.z2) && sl.id != slab.id);

    //     if max_z == slab.z2 || (supp_z1.is_some() && supp_z2.is_some()) {
    //         counter += 1;
    //     }
    // }
    
    let mut by_height = map.sand_slabs.iter()
        .map(|sl| (sl.1.z2, sl.0))
        .collect::<BTreeSet<_>>();

    let mut removable = (0..map.sand_slabs.len()).collect::<HashSet<_>>();

    let mut ground = BTreeSet::<(isize, usize)>::new();
    while let Some((_, fb_index)) = by_height.pop_first() {
        let mut saved_height = 0;
        let mut current_bottom = 0;
        let mut supporter = Vec::new();
        for (gb_top, gb_index) in ground.iter().rev() {
            if saved_height > 0 && *gb_top < current_bottom {
                break;
            }
            if map.sand_slabs[fb_index].collides_xy(&map.sand_slabs[gb_index]) {
                let height = map.sand_slabs[fb_index].z2 - map.sand_slabs[fb_index].z1;
                saved_height = gb_top + 1 + height;
                current_bottom = *gb_top;
                supporter.push(*gb_index);
            }
        }
        if saved_height == 0 {
            let height = map.sand_slabs[fb_index].z2 - map.sand_slabs[fb_index].z1 + 1;
            ground.insert((height, *fb_index));
        } else {
            ground.insert((saved_height, *fb_index));
        }
        if supporter.len() == 1 {
            removable.remove(&supporter[0]);
        }
    }

    println!("Part 1 answer: {}", removable.len());
    return Ok(());
}

fn part2(map: &Map) -> Result<()> {

    println!("Part 2 answer: {}", -1);
    return Ok(());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cubes_collide() {
        let cube1 = Coord { id: 1, x1: 1, y1: 1, z1: 1, x2: 4, y2: 4, z2: 4 };
        let cube2 = Coord { id: 2, x1: 3, y1: 4, z1: 1, x2: 6, y2: 6, z2: 1 };
        assert!(cube1.collides_xy(&cube2));
    }

    #[test]
    fn test_cubes_not_collide() {
        let cube1 = Coord { id: 1, x1: 1, y1: 1, z1: 1, x2: 4, y2: 4, z2: 4 };
        let cube2 = Coord { id: 2, x1: 5, y1: 6, z1: 1, x2: 6, y2: 6, z2: 1 };
        assert!(!cube1.collides_xy(&cube2));
    }
}

/*--- Day 22: Sand Slabs ---
Enough sand has fallen; it can finally filter water for Snow Island.

Well, almost.

The sand has been falling as large compacted bricks of sand, piling up to form an impressive stack here near the edge of Island Island. In order to make use of the sand to filter water, some of the bricks will need to be broken apart - nay, disintegrated - back into freely flowing sand.

The stack is tall enough that you'll have to be careful about choosing which bricks to disintegrate; if you disintegrate the wrong brick, large portions of the stack could topple, which sounds pretty dangerous.

The Elves responsible for water filtering operations took a snapshot of the bricks while they were still falling (your puzzle input) which should let you work out which bricks are safe to disintegrate. For example:

1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
Each line of text in the snapshot represents the position of a single brick at the time the snapshot was taken. The position is given as two x,y,z coordinates - one for each end of the brick - separated by a tilde (~). Each brick is made up of a single straight line of cubes, and the Elves were even careful to choose a time for the snapshot that had all of the free-falling bricks at integer positions above the ground, so the whole snapshot is aligned to a three-dimensional cube grid.

A line like 2,2,2~2,2,2 means that both ends of the brick are at the same coordinate - in other words, that the brick is a single cube.

Lines like 0,0,10~1,0,10 or 0,0,10~0,1,10 both represent bricks that are two cubes in volume, both oriented horizontally. The first brick extends in the x direction, while the second brick extends in the y direction.

A line like 0,0,1~0,0,10 represents a ten-cube brick which is oriented vertically. One end of the brick is the cube located at 0,0,1, while the other end of the brick is located directly above it at 0,0,10.

The ground is at z=0 and is perfectly flat; the lowest z value a brick can have is therefore 1. So, 5,5,1~5,6,1 and 0,2,1~0,2,5 are both resting on the ground, but 3,3,2~3,3,3 was above the ground at the time of the snapshot.

Because the snapshot was taken while the bricks were still falling, some bricks will still be in the air; you'll need to start by figuring out where they will end up. Bricks are magically stabilized, so they never rotate, even in weird situations like where a long horizontal brick is only supported on one end. Two bricks cannot occupy the same position, so a falling brick will come to rest upon the first other brick it encounters.

Here is the same example again, this time with each brick given a letter so it can be marked in diagrams:

1,0,1~1,2,1   <- A
0,0,2~2,0,2   <- B
0,2,3~2,2,3   <- C
0,0,4~0,2,4   <- D
2,0,5~2,2,5   <- E
0,1,6~2,1,6   <- F
1,1,8~1,1,9   <- G
At the time of the snapshot, from the side so the x axis goes left to right, these bricks are arranged like this:

 x
012
.G. 9
.G. 8
... 7
FFF 6
..E 5 z
D.. 4
CCC 3
BBB 2
.A. 1
--- 0
Rotating the perspective 90 degrees so the y axis now goes left to right, the same bricks are arranged like this:

 y
012
.G. 9
.G. 8
... 7
.F. 6
EEE 5 z
DDD 4
..C 3
B.. 2
AAA 1
--- 0
Once all of the bricks fall downward as far as they can go, the stack looks like this, where ? means bricks are hidden behind other bricks at that location:

 x
012
.G. 6
.G. 5
FFF 4
D.E 3 z
??? 2
.A. 1
--- 0
Again from the side:

 y
012
.G. 6
.G. 5
.F. 4
??? 3 z
B.C 2
AAA 1
--- 0
Now that all of the bricks have settled, it becomes easier to tell which bricks are supporting which other bricks:

Brick A is the only brick supporting bricks B and C.
Brick B is one of two bricks supporting brick D and brick E.
Brick C is the other brick supporting brick D and brick E.
Brick D supports brick F.
Brick E also supports brick F.
Brick F supports brick G.
Brick G isn't supporting any bricks.
Your first task is to figure out which bricks are safe to disintegrate. A brick can be safely disintegrated if, after removing it, no other bricks would fall further directly downward. Don't actually disintegrate any bricks - just determine what would happen if, for each brick, only that brick were disintegrated. Bricks can be disintegrated even if they're completely surrounded by other bricks; you can squeeze between bricks if you need to.

In this example, the bricks can be disintegrated as follows:

Brick A cannot be disintegrated safely; if it were disintegrated, bricks B and C would both fall.
Brick B can be disintegrated; the bricks above it (D and E) would still be supported by brick C.
Brick C can be disintegrated; the bricks above it (D and E) would still be supported by brick B.
Brick D can be disintegrated; the brick above it (F) would still be supported by brick E.
Brick E can be disintegrated; the brick above it (F) would still be supported by brick D.
Brick F cannot be disintegrated; the brick above it (G) would fall.
Brick G can be disintegrated; it does not support any other bricks.
So, in this example, 5 bricks can be safely disintegrated.

Figure how the blocks will settle based on the snapshot. Once they've settled, consider disintegrating a single brick; how many bricks could be safely chosen as the one to get disintegrated? */