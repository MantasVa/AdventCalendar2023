use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fs, hash::Hash};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// Coords, dir, steps
//dir: up, down, left, right
#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
struct Node(Coord, i64, i64);

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
struct Coord(i64, i64);

fn main() -> Result<()> {
    let nodes = parse()?;
    let max_x = nodes.iter().max_by(|a, b| a.0.0.cmp(&b.0.0)).unwrap().0.0;
    let max_y = nodes.iter().max_by(|a, b| a.0.1.cmp(&b.0.1)).unwrap().0.1;

    part1(&nodes, max_x, max_y)?;
    part2(&nodes, max_x, max_y)?;

    return Ok(());
}

fn parse() -> Result<HashMap<Coord, i64>> {
    let input = fs::read_to_string("input.txt")?;

    let mut nodes: HashMap<Coord, i64> = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            nodes.insert(Coord(x as i64, y as i64), ch.to_string().parse().unwrap());
        }
    }

    Ok(nodes)
}

fn part1(nodes: &HashMap<Coord, i64>, max_x: i64, max_y: i64) -> Result<()> {
    let start = Node(Coord(0, 0), 0, 0);
    let goal = Coord(max_x, max_y);

    let answer = dijkstra(&start, |node| {
        let x = node.0.0;
        let y = node.0.1;

        let dir = node.1;
        let steps = node.2;

        let up_coord = Coord(x - 1, y);
        let down_coord = Coord(x + 1, y);
        let left_coord = Coord(x, y- 1);
        let right_coord = Coord(x, y + 1);

        let mut neighbors: Vec<(Node, i64)> = Vec::new();

        if dir == 0 && steps < 3 {
            neighbors.push((Node(up_coord.clone(), 0, steps + 1), *nodes.get(&up_coord).unwrap_or(&1000)))
        } else if dir != 0 && dir != 1 {
            neighbors.push((Node(up_coord.clone(), 0, 1), *nodes.get(&up_coord).unwrap_or(&1000)))
        }

        if dir == 1 && steps < 3 {
            neighbors.push((Node(down_coord.clone(), 1, steps + 1), *nodes.get(&down_coord).unwrap_or(&1000)))
        } else if dir != 1 && dir != 0 {
            neighbors.push((Node(down_coord.clone(), 1, 1), *nodes.get(&down_coord).unwrap_or(&1000)))
        }

        if dir == 2 && steps < 3 {
            neighbors.push((Node(left_coord.clone(), 2, steps + 1), *nodes.get(&left_coord).unwrap_or(&1000)))
        } else if dir != 2 && dir != 3 {
            neighbors.push((Node(left_coord.clone(), 2, 1), *nodes.get(&left_coord).unwrap_or(&1000)))
        }

        if dir == 3 && steps < 3 {
            neighbors.push((Node(right_coord.clone(), 3, steps + 1), *nodes.get(&right_coord).unwrap_or(&1000)))
        } else if dir != 3 && dir != 2 {
            neighbors.push((Node(right_coord.clone(), 3, 1), *nodes.get(&right_coord).unwrap_or(&1000)))
        }

        neighbors
    }, |node| node.0 == goal);

    println!("Part 1 answer: {}", answer.iter().next().unwrap().1);
    return Ok(());
}

fn part2(nodes: &HashMap<Coord, i64>, max_x: i64, max_y: i64) -> Result<()> {
    let start = Node(Coord(0, 0), -1, 0);
    let goal = Coord(max_x, max_y);

    let answer = dijkstra(&start, |node| {
        let x = node.0.0;
        let y = node.0.1;

        let dir = node.1;
        let steps = node.2;

        let up_coord = Coord(x - 1, y);
        let down_coord = Coord(x + 1, y);
        let left_coord = Coord(x, y- 1);
        let right_coord = Coord(x, y + 1);

        let mut neighbors: Vec<(Node, i64)> = Vec::new();

        if (dir == 0 || dir == -1) && steps < 10 {
            neighbors.push((Node(up_coord.clone(), 0, steps + 1), *nodes.get(&up_coord).unwrap_or(&1000)))
        } else if dir != 0 && dir != 1 && steps > 3 {
            neighbors.push((Node(up_coord.clone(), 0, 1), *nodes.get(&up_coord).unwrap_or(&1000)))
        }

        if (dir == 1 || dir == -1) && steps < 10 {
            neighbors.push((Node(down_coord.clone(), 1, steps + 1), *nodes.get(&down_coord).unwrap_or(&1000)))
        } else if dir != 1 && dir != 0 && steps > 3 {
            neighbors.push((Node(down_coord.clone(), 1, 1), *nodes.get(&down_coord).unwrap_or(&1000)))
        }

        if (dir == 2 || dir == -1) && steps < 10 {
            neighbors.push((Node(left_coord.clone(), 2, steps + 1), *nodes.get(&left_coord).unwrap_or(&1000)))
        } else if dir != 2 && dir != 3 && steps > 3 {
            neighbors.push((Node(left_coord.clone(), 2, 1), *nodes.get(&left_coord).unwrap_or(&1000)))
        }

        if (dir == 3 || dir == -1) && steps < 10 {
            neighbors.push((Node(right_coord.clone(), 3, steps + 1), *nodes.get(&right_coord).unwrap_or(&1000)))
        } else if dir != 3 && dir != 2 && steps > 3 {
            neighbors.push((Node(right_coord.clone(), 3, 1), *nodes.get(&right_coord).unwrap_or(&1000)))
        }

        neighbors
    }, |node| node.0 == goal && node.2 > 3);

    println!("Part 2 answer: {}", answer.iter().next().unwrap().1);
    return Ok(());
}