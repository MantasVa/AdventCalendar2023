use std::{fs, collections::HashSet, vec, ops::Range};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct TranslationTable {
    seeds: Vec<i64>,
    seed_soil: Vec<RangeMap>,
    soil_fertilizer: Vec<RangeMap>,
    fertilizer_water: Vec<RangeMap>,
    water_light: Vec<RangeMap>,
    light_temperature: Vec<RangeMap>,
    temperature_humidity: Vec<RangeMap>,
    humidity_location: Vec<RangeMap>,
}

impl TranslationTable {
    fn new() -> TranslationTable {
        TranslationTable { seeds: vec![], seed_soil: vec![], soil_fertilizer: vec![], 
            fertilizer_water: vec![], water_light: vec![], light_temperature: vec![], 
            temperature_humidity: vec![], humidity_location: vec![]}
    }

    fn get_seed_location(&self, seed: i64) -> i64 {
        let mut soil = seed;
        for range_map in &self.seed_soil {
            if range_map.range.contains(&seed) {
                soil = seed + range_map.delta;
                break;
            }
        }

        let mut fertilizer = soil;
        for range_map in &self.soil_fertilizer {
            if range_map.range.contains(&soil) {
                fertilizer = soil + range_map.delta;
                break;
            }
        }

        let mut water = fertilizer;
        for range_map in &self.fertilizer_water {
            if range_map.range.contains(&fertilizer) {
                water = fertilizer + range_map.delta;
                break;
            }
        }

        let mut light = water;
        for range_map in &self.water_light {
            if range_map.range.contains(&water) {
                light = water + range_map.delta;
                break;
            }
        }

        let mut temperature = light;
        for range_map in &self.light_temperature {
            if range_map.range.contains(&light) {
                temperature = light + range_map.delta;
                break;
            }
        }

        let mut humidity = temperature;
        for range_map in &self.temperature_humidity {
            if range_map.range.contains(&temperature) {
                humidity = temperature + range_map.delta;
                break;
            }
        }

        let mut location = humidity;
        for range_map in &self.humidity_location {
            if range_map.range.contains(&humidity) {
                location = humidity + range_map.delta;
                break;
            }
        }

        location
    }
}

struct RangeMap {
    range: Range<i64>,
    delta: i64,
}

fn main() -> Result<()> {
    let translation_table = parse()?;

    let _result1 = part1(&translation_table);
    let result2 = part2(&translation_table);

    result2
}

fn parse() -> Result<TranslationTable> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.split("\r\n\r\n").collect::<Vec<_>>();

    let mut table = TranslationTable::new();
    for group in input {
        if group.starts_with("seeds") {
            table.seeds = group.split_once(':')
            .unwrap().1.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        }
        else {
            let group = group.split("\r\n").collect::<Vec<_>>();

            let name = group[0];
            let translations = group[1..].iter().collect::<Vec<_>>();

            for translation in translations {
                let translation = translation.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();

                let source = translation[1];
                let destination = translation[0];
                let len = translation[2];

                let range = source..source + len;
                let delta = destination - source;

                match name {
                    _ if name.starts_with("seed-to-soil") => table.seed_soil.push(RangeMap { range, delta }),
                    _ if name.starts_with("soil-to-fertilizer") => table.soil_fertilizer.push(RangeMap {range, delta}),
                    _ if name.starts_with("fertilizer-to-water") => table.fertilizer_water.push(RangeMap {range, delta}),
                    _ if name.starts_with("water-to-light") => table.water_light.push(RangeMap {range, delta}),
                    _ if name.starts_with("light-to-temperature") => table.light_temperature.push(RangeMap {range, delta}),
                    _ if name.starts_with("temperature-to-humidity") => table.temperature_humidity.push(RangeMap {range, delta}),
                    _ if name.starts_with("humidity-to-location") => table.humidity_location.push(RangeMap {range, delta}),
                    _ => ()
                };
            }
        }
    }

    Ok(table)
}

fn part1(table: &TranslationTable) -> Result<()> {
    let min_location = table.seeds.iter().map(|s | table.get_seed_location(*s)).min().unwrap();
    
    println!("Part 1 answer: {}", min_location);
    return Ok(());
}

fn part2(table: &TranslationTable) -> Result<()> {
    let min_location = table.seeds.iter().collect::<Vec<_>>().chunks(2)
    .flat_map(|x| (*x[0]..*x[0]+*x[1]).clone().collect::<Vec<i64>>())
    .map(|s | table.get_seed_location(s)).min().unwrap();

    println!("Part 2 answer: {}", min_location);
    return Ok(());
}

/*--- Day 5: If You Give A Seed A Fertilizer ---
You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48
The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51
With this map, you can look up the soil number required for each initial seed number:

Seed number 79 corresponds to soil number 81.
Seed number 14 corresponds to soil number 14.
Seed number 55 corresponds to soil number 57.
Seed number 13 corresponds to soil number 13.
The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial seed numbers?

--- Part Two ---
Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

seeds: 79 14 55 13
This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers? */