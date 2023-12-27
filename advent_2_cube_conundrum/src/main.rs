use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Set{
    blue: i32, 
    red: i32, 
    green: i32
}

fn main() -> Result<()> {
    let _result1 = part1();
    let result2 = part2();

    result2
}

fn part1() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    const BAG_TOTAL: (i32, i32, i32) = (14, 12, 13);
    let mut id_total = 0;

    for line in input.lines() {
        let splits = line.split(':').collect::<Vec<&str>>();

        let id: i32 = splits[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap_or(0);
        let rounds = splits[1].split(';').collect::<Vec<&str>>();

        let sets = rounds.iter().map(|r| to_set(r.trim())).collect::<Vec<Set>>();

        let blues = sets.iter().max_by(|x, y| x.blue.cmp(&y.blue)).unwrap().blue;
        let reds = sets.iter().max_by(|x, y| x.red.cmp(&y.red)).unwrap().red;
        let greens = sets.iter().max_by(|x, y| x.green.cmp(&y.green)).unwrap().green;

        if BAG_TOTAL.0 >= blues && BAG_TOTAL.1 >= reds && BAG_TOTAL.2 >= greens {
            id_total += id;
        }
    }

    println!("Part 1 answer: {}", id_total);
    return Ok(());
}

fn part2() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut power_total = 0;

    for line in input.lines() {
        let splits = line.split(':').collect::<Vec<&str>>();

        let id: i32 = splits[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap_or(0);
        let rounds = splits[1].split(';').collect::<Vec<&str>>();

        let sets = rounds.iter().map(|r| to_set(r.trim())).collect::<Vec<Set>>();

        let blues = sets.iter().max_by(|x, y| x.blue.cmp(&y.blue)).unwrap().blue;
        let reds = sets.iter().max_by(|x, y| x.red.cmp(&y.red)).unwrap().red;
        let greens = sets.iter().max_by(|x, y| x.green.cmp(&y.green)).unwrap().green;

        power_total += blues * reds * greens;
    }

    println!("Part 2 answer: {}", power_total);
    return Ok(());
}

fn to_set(s: &str) -> Set {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;


    for cube in s.split(',') {
        let spl = cube.split(' ').filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

        let count = spl[0].trim().parse::<i32>().unwrap();
        let color = spl[1].trim();

        if color == "blue" {
            blue = count;
        }
        if color == "red" {
            red = count;
        }
        if color == "green" {
            green = count;
        }
    }

    Set { blue, red, green}
}
