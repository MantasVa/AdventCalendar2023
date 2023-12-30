use std::fs;
use regex::Regex;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct EngineSchema {
    nums: Vec<(u32, (i32, i32, i32))>,
    syms: Vec<(char, (i32, i32))>,
    gears: Vec<(i32, i32)>,
}

fn main() -> Result<()> {
    let schema = parse()?;
    let _ = part1(&schema);
    part2(&schema)
}

fn parse() -> Result<EngineSchema> {
    let input = fs::read_to_string("input.txt")?;

    let num_regex = Regex::new(r"(\d+)")?;
    let sym_regex = Regex::new(r"[^.\d\s]")?;

    let mut nums: Vec<(u32, (i32, i32, i32))> = vec![];
    let mut syms: Vec<(char, (i32, i32))> = vec![];
    let mut gears: Vec<(i32, i32)> = vec![];
    for (row, line) in input.lines().enumerate() {
        for cap in num_regex.find_iter(line.trim()) {
            let number = cap.as_str().parse::<u32>().unwrap();

            let last = nums.iter().last();
            let col: i32;
            if last.is_some() && last.unwrap().1.0 as usize == row {
                if let Some(n) = line.get((last.unwrap().1.1 as usize + 1)..).and_then(|s| s.find(cap.as_str()).map(|i| i + last.unwrap().1.1 as usize + 1)) {
                    col = n as i32;
                }
                else {
                    col = line.find(cap.as_str()).unwrap() as i32;
                }
            }
            else {
                col = line.find(cap.as_str()).unwrap() as i32;
            }

            nums.push((number, (row as i32, col, cap.as_str().len() as i32)));
        }

        for cap in sym_regex.find_iter(line) {
            let symbol = cap.as_str().parse::<char>().unwrap();
            
            let last = syms.iter().last();
            let col: i32;
            if last.is_some() && last.unwrap().1.0 as usize == row {
                if let Some(n) = line.get((last.unwrap().1.1 as usize + 1)..).and_then(|s| s.find(symbol).map(|i| i + last.unwrap().1.1 as usize + 1)) {
                    col = n as i32;
                }
                else {
                    col = line.find(symbol).unwrap() as i32;
                }
            }
            else {
                col = line.find(symbol).unwrap() as i32;
            }

            syms.push((symbol, (row as i32, col)));

            if symbol == '*' {
                gears.push((row as i32, col));
            }
        }
    }

    Ok(EngineSchema { nums, syms, gears })
}

fn part1(schema: &EngineSchema) -> Result<()> {
    let mut total: u32 = 0;
    for (number, (row, col, num_len)) in &schema.nums {
        let (start, end) = (col, col + num_len - 1);

        let has_adjec_symbol = &schema.syms.iter().any(|(_, (s_row, s_col))| 
        (*s_row == *row && *s_col == start - 1 ) ||
        (*s_row == *row && *s_col == end + 1 ) ||
        (*s_row == row - 1 && (*s_col >= start - 1 && *s_col <= end + 1)) || 
        (*s_row == row + 1 && (*s_col >= start - 1 && *s_col <= end + 1)));

        if *has_adjec_symbol {
            total += number;
        }
    }

    println!("Part 1 answer: {}", total);
    return Ok(());
}

fn part2(schema: &EngineSchema) -> Result<()> {
    let mut total: u32 = 0;
    for (row, col) in &schema.gears {
        let adjec_nums = &schema.nums.iter().filter(|(_, (n_row, n_col, n_col_len))| 
            (*n_row == *row && *n_col == col - 1 ) ||
            (*n_row == *row && *n_col + n_col_len - 1 == col - 1 ) ||
            (*n_row == *row && *n_col == col + 1 ) ||
            (*n_row == *row && *n_col + n_col_len - 1 == col + 1 ) ||
            (*n_row == row - 1 && ((*n_col >= col - 1 && *n_col <= col + 1) || (*n_col + n_col_len -1 >= col - 1 && *n_col + n_col_len -1 <= col + 1))) || 
            (*n_row == row + 1 && ((*n_col >= col - 1 && *n_col <= col + 1) || (*n_col + n_col_len -1 >= col - 1 && *n_col + n_col_len -1 <= col + 1))))
            .collect::<Vec<&(u32, (i32, i32, i32))>>();

        if adjec_nums.len() == 2 {
            total += adjec_nums.first().unwrap().0 * adjec_nums.last().unwrap().0;
        }
    }

    println!("Part 2 answer: {}", total);
    return Ok(());
}

/*--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic? */