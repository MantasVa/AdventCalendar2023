use std::fs;
use regex::Regex;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let _result1 = part1();
    let result2 = part2();

    result2
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut sum: u32 = 0;
    for line in input.lines() {
        if !line.is_empty() {
            let mut left = 0;
            let mut right = 0;

            const RADIX: u32 = 10;

            for c in line.chars() {
                if c.is_numeric() {
                    left = c.to_digit(RADIX).unwrap_or(0);
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_numeric() {
                    right = c.to_digit(RADIX).unwrap_or(0);
                    break;
                }
            } 

            sum += left * 10 + right;
        }
    }

    println!("Part 1 answer: {}", sum);
    return Ok(());
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let start_regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*$")?;
    let end_regex = Regex::new(r"^.*(\d|one|two|three|four|five|six|seven|eight|nine)")?;

    let mut sum: u32 = 0;
    for line in input.lines() {
        if !line.is_empty() {
            let Some(captures_start) = start_regex.captures(line) else {
                panic!("Number not found")
            };
            let Some(captures_end) = end_regex.captures(line) else {
                panic!("Number not found")
            };

            let capture_start = captures_start[1].parse::<String>().unwrap();
            let left = parse_number(&capture_start);

            let capture_end = captures_end[1].parse::<String>().unwrap();
            let right = parse_number(&capture_end);

            sum += left * 10 + right;
        }
    }

    println!("Part 2 answer: {}", sum);
    return Ok(());
}

fn parse_number(n: &str) -> u32 {
    match n {
        n if n.chars().next().unwrap().is_numeric() => n.parse().unwrap(),
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Number not converted")
    }
}
