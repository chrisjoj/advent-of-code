use itertools::Itertools;
use crate::custom_error::AocError;
use std::borrow::Borrow;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u32, AocError> {
    const SEEDS_SECTION: &str = "seeds:";
    const SEED_MAPPING: &str = "seed-t";
    const SOIL_MAPPING: &str = "soil-t";
    const FERT_MAPPING: &str = "fertil";
    const WATER_MAPPING: &str = "water-";
    const LIGHT_MAPPING: &str = "light-";
    const TEMP_MAPPING: &str = "temper";
    const HUMIDITY_MAPPING: &str = "humidi";

    let seeds_iterator = vec![];
    let seed_to_soil = vec![];
    let soil_to_fert = vec![];
    let fert_to_water = vec![];
    let water_to_light = vec![];
    let light_to_temp = vec![];
    let temp_to_humidity = vec![];
    let humidity_to_location = vec![];

    let mut mappings = [seeds_iterator, seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp, temp_to_humidity, humidity_to_location];
    let mut index = 0;

    _input.lines().for_each(|line| {
        match Some(line) {
            Some("") => {}
            Some(x) if &x[0..6] == SEEDS_SECTION => {
                let range: Vec<_> = line[7..].split_ascii_whitespace().chunks(2).into_iter().map(|mut part| {
                    let start = part.next().unwrap().parse::<u32>().expect("Expected first number");
                    let range = part.next().unwrap().parse::<u32>().expect("Expected first number");
                    let seeds = Mapping {
                        source: start,
                        range: range,
                        destination: start,
                    };
                    seeds
                }).collect_vec();
                println!("Found seeds adding: {:?}",range);
                range.iter().for_each(|r| {
                    let mapping = mappings.get_mut(0).unwrap();
                    mapping.push(*r);
                });
            }

            Some(x) if &x[0..6] == SEED_MAPPING || &x[0..6] == SOIL_MAPPING || &x[0..6] == FERT_MAPPING || &x[0..6] == WATER_MAPPING || &x[0..6] == LIGHT_MAPPING || &x[0..6] == HUMIDITY_MAPPING || &x[0..6] == TEMP_MAPPING
            => { index += 1 }
            _ => {
                let mut parts = line.split_ascii_whitespace();
                let mapping = mappings.get_mut(index).unwrap();

                let destination = parts.next().unwrap().parse::<u32>().expect("Destination should be a valid number");
                let source = parts.next().unwrap().parse::<u32>().expect("Source should be a valid number");
                let range = parts.next().unwrap().parse::<u32>().expect("Range should be a valid number");

                let map = Mapping {
                    destination,
                    source,
                    range,
                };
                let _ = mapping.push(map);
            }
        }
    });

    let result = mappings.iter().fold((0, mappings[0].clone()), |acc, mapping| {
        let index = acc.0;
        let this = acc.1;
        match index {
            0 => { (1, this) }
            _ => {
                let next = mapping;

                let new_ranges = this.iter().map(|mapping| {
                    let a = next.iter().map(|next_mapping| {
                        // generate new ranges from comparision
                        let m = Mapping { source: 1, destination: 1, range: 1 };
                        m
                    }).collect::<Vec<Mapping>>();
                    a
                }).collect::<Vec<_>>().into_iter().flatten().collect::<Vec<Mapping>>();
                (index.clone() + 1, new_ranges)
            }
        }
    }).1.into_iter().min();

    println!("{:?}", result);

    Ok(0)
}


#[derive(Debug, Eq, Copy, Clone, PartialEq, Ord, PartialOrd)]
struct Mapping {
    source: u32,
    destination: u32,
    range: u32,
}

impl Mapping {
    fn destination_offset(&self) -> i32 {
        (self.destination - self.source) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
