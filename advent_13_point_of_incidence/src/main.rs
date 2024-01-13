use std::{fs, collections::HashMap, ops::Range};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct Valley {
    patterns: Vec<Pattern>
}

impl Valley {
    fn get_result(&self, has_error: bool) -> i64 {
        let mut result: i64 = 0;

        for pattern in &self.patterns {
            let mut idx = 0;
            let mut found = false;
    
            while idx < pattern.rows {
                let (is_equal, err) = pattern.is_rows_equal(idx, idx + 1, has_error);
                
                if is_equal {
                    if let Some((range_l, range_r)) = Pattern::get_ranges(idx, pattern.rows){
                        let reflections = range_l.zip(range_r.rev()).map(|(row_1, row_2)| pattern.is_rows_equal(row_1, row_2, has_error)).collect::<Vec<_>>();

                        if reflections.iter().all(|r| r.0) && reflections.iter().map(|r| r.1).sum::<i64>() + err <= 1 {
                            result += (idx + 1)  * 100;
                            found = true;
                            break;
                        }
                    } else { 
                        result += (idx + 1)  * 100;
                        found = true;
                        break;
                    }
                }
    
                idx += 1;
            }
    
            idx = 0;
            while !found && idx < pattern.cols {
                let (is_equal, err) = pattern.is_cols_equal(idx, idx + 1, has_error);
                
                if is_equal {
                    if let Some((range_l, range_r)) = Pattern::get_ranges(idx, pattern.cols){
                        let reflections = range_l.zip(range_r.rev()).map(|(col_1, col_2)| pattern.is_cols_equal(col_1, col_2, has_error)).collect::<Vec<_>>();
                        
                        if reflections.iter().all(|r| r.0) && reflections.iter().map(|r| r.1).sum::<i64>() + err <= 1 {
                            result += idx + 1;
                            break;
                        }
                    } else { 
                        result += idx + 1;
                        break;
                    }
                }
                idx += 1;
            }
        }

        result
    }
}

struct Pattern {
    rows: i64,
    cols: i64,
    elements: HashMap<(i64, i64), Type>
}

impl Pattern {

    fn is_rows_equal(&self, row_1: i64, row_2: i64, has_error: bool) -> (bool, i64) {
        if row_1 >= self.rows || row_2 > self.rows {
            panic!("Not enough rows in pattern");
        }

        if has_error {
            let are_equal = (0..self.cols + 1).map(|col| *self.elements.get(&(row_1, col)).unwrap() == *self.elements.get(&(row_2, col)).unwrap()).collect::<Vec<_>>();
            let neq_count = are_equal.len() - are_equal.iter().filter(|b| **b).count();
            (neq_count <= 1, neq_count as i64)
        } else {
            ((0..self.cols + 1).all(|col| *self.elements.get(&(row_1, col)).unwrap() == *self.elements.get(&(row_2, col)).unwrap()), 0)
        }
    }

    fn is_cols_equal(&self, col_1: i64, col_2: i64, has_error: bool) -> (bool, i64) {
        if col_1 >= self.cols || col_2 > self.cols {
            panic!("Not enough cols in pattern");
        }

        if has_error {
            let are_equal = (0..self.rows + 1).map(|row| *self.elements.get(&(row, col_1)).unwrap() == *self.elements.get(&(row, col_2)).unwrap()).collect::<Vec<_>>();
            let neq_count = are_equal.len() - are_equal.iter().filter(|b| **b).count();
            (neq_count <= 1, neq_count as i64)

        } else {
            ((0..self.rows + 1).all(|row| *self.elements.get(&(row, col_1)).unwrap() == *self.elements.get(&(row, col_2)).unwrap()), 0)
        }
    }

    fn get_ranges(i: i64, max: i64) -> Option<(Range<i64>, Range<i64>)> {
        let last_idx;
        if  i * 2 + 1 > max && i + 2 <= max {
            last_idx = max
        } else if i * 2 + 1 < max  {
            last_idx = i * 2 + 1;
        } else {
            return None;
        }

        let first_idx;
        if last_idx - i - 1 < i {
            first_idx = i - (last_idx - i - 1)
        }
        else {
            first_idx = 0;
        }

        Some((first_idx..i, i + 2..last_idx + 1))
    }
}

#[derive(PartialEq, Eq)]
enum Type {
    Rock,
    Ash
}

impl Type {
    fn new(c: char) -> Type {
        match c {
            '#' => Type::Rock,
            '.' => Type::Ash,
            _ => panic!("Bad input")
        }
    }
}

fn main() -> Result<()> {
    let valley = parse()?;
    _ = part1(&valley)?;
    _ = part2(&valley)?;

    return Ok(());
}

fn parse() -> Result<Valley> {
    let input = fs::read_to_string("input.txt")?;
    let splits = input.split("\r\n\r\n");

    let mut patterns: Vec<Pattern> = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for map in splits {
        let mut elements: HashMap<(i64, i64), Type> = HashMap::new();

        for (x, row) in map.split("\r\n").enumerate() {
            rows = x as i64;
            for (y, c) in row.chars().enumerate() {
                elements.insert((x as i64, y as i64), Type::new(c));

                cols = y as i64;
            }
        }
        
        patterns.push(Pattern { rows, cols, elements })
    }

    Ok(Valley { patterns })
}

fn part1(valley: &Valley) -> Result<()> {
    let result = valley.get_result(false);
    println!("Part 1 answer: {}", result);
    return Ok(());
}

fn part2(valley: &Valley) -> Result<()> {
    let result = valley.get_result(true);
    println!("Part 2 answer: {}", result);
    return Ok(());
}

/* --- Day 13: Point of Incidence ---
With your help, the hot springs team locates an appropriate spring which launches you neatly and precisely up to the edge of Lava Island.

There's just one problem: you don't see any lava.

You do see a lot of ash and igneous rock; there are even what look like gray mountains scattered around. After a while, you make your way to a nearby cluster of mountains only to discover that the valley between them is completely full of large mirrors. Most of the mirrors seem to be aligned in a consistent way; perhaps you should head in that direction?

As you move through the valley of mirrors, you find that several of them have fallen from the large metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of the fallen mirrors have lodged into the ash at strange angles. Because the terrain is all one color, it's hard to tell where it's safe to walk or where you're about to run into a mirror.

You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

For example:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns.

In the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:

123456789
    ><   
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><   
123456789
In this pattern, the line of reflection is the vertical line between columns 5 and 6. Because the vertical line is not perfectly in the middle of the pattern, part of the pattern (column 1) has nowhere to reflect onto and can be ignored; every other column has a reflected column within the pattern and must match exactly: column 2 matches column 9, column 3 matches 8, 4 matches 7, and 5 matches 6.

The second pattern reflects across a horizontal line instead:

1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7
This pattern reflects across the horizontal line between rows 4 and 5. Row 1 would reflect with a hypothetical row 8, but since that's not in the pattern, row 1 doesn't need to match anything. The remaining rows match: row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.

To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.

Find the line of reflection in each of the patterns in your notes. What number do you get after summarizing all of your notes?

--- Part Two ---
You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing.

Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.

In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

Here's the above example again:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
The first pattern's smudge is in the top-left corner. If the top-left # were instead ., it would have a different, horizontal line of reflection:

1 ..##..##. 1
2 ..#.##.#. 2
3v##......#v3
4^##......#^4
5 ..#.##.#. 5
6 ..##..##. 6
7 #.#.##.#. 7
With the smudge in the top-left corner repaired, a new horizontal line of reflection between rows 3 and 4 now exists. Row 7 has no corresponding reflected row and can be ignored, but every other row matches exactly: row 1 matches row 6, row 2 matches row 5, and row 3 matches row 4.

In the second pattern, the smudge can be fixed by changing the fifth symbol on row 2 from . to #:

1v#...##..#v1
2^#...##..#^2
3 ..##..### 3
4 #####.##. 4
5 #####.##. 5
6 ..##..### 6
7 #....#..# 7
Now, the pattern has a different horizontal line of reflection between rows 1 and 2.

Summarize your notes as before, but instead use the new different reflection lines. In this example, the first pattern's new horizontal line has 3 rows above it and the second pattern's new horizontal line has 1 row above it, summarizing to the value 400.

In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?*/