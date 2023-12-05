use itertools::Itertools;
use std::ops::Range;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

struct MapEntry {
    dest_start: i64,
    source_range: Range<i64>,
}

impl MapEntry {
    fn from_row(row: &str) -> Self {
        let (dest, remainder) = row.split_once(" ").unwrap();
        let (source, len) = remainder.split_once(" ").unwrap();
        let source = source.parse().unwrap();
        let len: i64 = len.parse().unwrap();
        Self {
            dest_start: dest.parse().unwrap(),
            source_range: source..(source + len),
        }
    }

    fn map_output(&self, source: &i64) -> Option<i64> {
        if self.source_range.contains(&source) {
            Some(self.dest_start + (source - self.source_range.start))
        } else {
            None
        }
    }
}

struct Map {
    map: Vec<MapEntry>,
}

impl Map {
    fn from_block(block: &str) -> Self {
        Self {
            map: block.lines().map(|line| MapEntry::from_row(line)).collect(),
        }
    }

    fn run_map(&self, source: i64) -> i64 {
        for entry in self.map.iter() {
            if let Some(out) = entry.map_output(&source) {
                return out;
            }
        }
        source
    }
}

struct Almanac {
    seed_to_soil: Map,
    soil_to_fert: Map,
    fert_to_water: Map,
    water_to_light: Map,
    light_to_temp: Map,
    temp_to_humid: Map,
    humid_to_loc: Map,
}

impl Almanac {
    fn from_input(input: &str) -> Self {
        let input = input
            .replace("seed-to-soil map:", "")
            .replace("soil-to-fertilizer map:", "")
            .replace("fertilizer-to-water map:", "")
            .replace("water-to-light map:", "")
            .replace("light-to-temperature map:", "")
            .replace("temperature-to-humidity map:", "")
            .replace("humidity-to-location map:", "");
        let mut blocks = input.split("\n\n\n");
        blocks.next(); // throw away the seeds line, don't need it here
        let seed_to_soil = Map::from_block(blocks.next().unwrap());
        let soil_to_fert = Map::from_block(blocks.next().unwrap());
        let fert_to_water = Map::from_block(blocks.next().unwrap());
        let water_to_light = Map::from_block(blocks.next().unwrap());
        let light_to_temp = Map::from_block(blocks.next().unwrap());
        let temp_to_humid = Map::from_block(blocks.next().unwrap());
        let humid_to_loc = Map::from_block(blocks.next().unwrap());
        assert_eq!(None, blocks.next()); // we should have gotten all blocks. Check.

        Self {
            seed_to_soil,
            soil_to_fert,
            fert_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humid,
            humid_to_loc,
        }
    }

    fn run_alamanac_map(&self, source: &i64) -> i64 {
        self.humid_to_loc.run_map(
            self.temp_to_humid.run_map(
                self.light_to_temp.run_map(
                    self.water_to_light.run_map(
                        self.fert_to_water.run_map(
                            self.soil_to_fert
                                .run_map(self.seed_to_soil.run_map(*source)),
                        ),
                    ),
                ),
            ),
        )
    }
}

fn part1(text: &str) {
    let almanac = Almanac::from_input(text);
    println!(
        "{}",
        text.lines()
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|seed| almanac.run_alamanac_map(&seed.parse().unwrap()))
            .min()
            .unwrap()
    );
}

fn part2(text: &str) {
    let almanac = Almanac::from_input(text);
    println!(
        "{}",
        text.lines()
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|seed| seed.parse::<i64>().unwrap())
            .tuples()
            .flat_map(|(start, len)| {
                (start..(start + len))
                    .into_iter()
                    .map(|seed| almanac.run_alamanac_map(&seed))
            })
            .min()
            .unwrap()
    );
}
