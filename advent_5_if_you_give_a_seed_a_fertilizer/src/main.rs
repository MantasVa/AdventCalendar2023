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
