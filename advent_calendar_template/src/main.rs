use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let _result1 = part1();
    let result2 = part2();

    result2
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    for line in input.lines() {
        
    }

    println!("Part 1 answer: {}", -1);
    return Ok(());
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    println!("Part 2 answer: {}", -1);
    return Ok(());
}
