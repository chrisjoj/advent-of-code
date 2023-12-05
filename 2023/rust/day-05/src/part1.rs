use std::collections::BTreeMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u32, AocError> {
    const seeds_section: &str = "seeds:";
    const seed_mapping: &str = "seed-t";
    const soil_mapping: &str = "soil-t";
    const fert_mapping: &str = "fertil";
    const water_mapping: &str = "water-";
    const light_mapping: &str = "light-";
    const temp_mapping: &str = "temper";
    const humidity_mapping: &str = "humidi";

    let mut seeds: Vec<u32> = vec![];
    let seed_to_soil: BTreeMap<u32, Mapping> = BTreeMap::new();
    let soil_to_fert: BTreeMap<u32, Mapping> = BTreeMap::new();
    let fert_to_water: BTreeMap<u32, Mapping> = BTreeMap::new();
    let water_to_light: BTreeMap<u32, Mapping> = BTreeMap::new();
    let light_to_temp: BTreeMap<u32, Mapping> = BTreeMap::new();
    let temp_to_humidity: BTreeMap<u32, Mapping> = BTreeMap::new();
    let humidity_to_location: BTreeMap<u32, Mapping> = BTreeMap::new();

    let mut mappings = [seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp, temp_to_humidity, humidity_to_location];
    let mut index = 0;
    _input.lines().for_each(|line| {
        match Some(line) {
            Some("") => {}
            Some(x) if &x[0..6] == seeds_section => {
                line.split_ascii_whitespace().for_each(|part| {
                    match part {
                        seeds_section => {}
                        &_ => {
                            let seed = part.parse::<u32>().unwrap();
                            seeds.push(seed);
                        }
                    }
                }
                );
            }
            Some(x) if &x[0..6] == seed_mapping => {}
            Some(x) if &x[0..6] == soil_mapping || &x[0..6] == fert_mapping || &x[0..6] == water_mapping || &x[0..6] == light_mapping || &x[0..6] == humidity_mapping || &x[0..6] == temp_mapping
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
                let _ = mapping.insert(source, map);
            }
        }
    });

    let location = seeds.iter().map(|seed| {
        let loc = mappings.iter().fold(*seed, |mut acc, mapping| *{
            let entry: Option<&u32> = mapping.keys().filter(|source| {
                source <= &&acc
            }).max();
            match entry {
                None => {
                    &acc }
                Some(k) => {
                    let diff = acc - k;
                    let range = mapping.get(k).unwrap().range;
                    match diff > range {
                        true => {&acc}
                        false => {
                            let dest = mapping.get(k).unwrap().destination;
                            acc = dest + diff;
                            &acc
                        }
                    }
                }
            }
        });
        loc
    }).min().expect("Should have returned a location");

    Ok(location)
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Mapping {
    source: u32,
    destination: u32,
    range: u32,
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
        assert_eq!(35, process(input)?);
        Ok(())
    }
}
