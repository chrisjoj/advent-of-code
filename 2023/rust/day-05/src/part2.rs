use itertools::Itertools;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u64, AocError> {
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

    let mut mappings = [seeds_iterator, seed_to_soil.clone(), soil_to_fert.clone(), fert_to_water, water_to_light, light_to_temp, temp_to_humidity, humidity_to_location];
    let mut index = 0;

    _input.lines().for_each(|line| {
        match Some(line) {
            Some("") => {}
            Some(x) if &x[0..6] == SEEDS_SECTION => {
                let range: Vec<_> = line[7..].split_ascii_whitespace().chunks(2).into_iter().map(|mut part| {
                    let start = part.next().unwrap().parse::<u64>().expect("Expected first number");
                    let range = part.next().unwrap().parse::<u64>().expect("Expected first number");
                    let seeds = Mapping {
                        source: start,
                        range: range,
                        destination: start,
                    };
                    seeds
                }).collect_vec();
                println!("Found seeds adding: {:?}", range);
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

                let destination = parts.next().unwrap().parse::<u64>().expect("Destination should be a valid number");
                let source = parts.next().unwrap().parse::<u64>().expect("Source should be a valid number");
                let range = parts.next().unwrap().parse::<u64>().expect("Range should be a valid number");

                let map = Mapping {
                    destination,
                    source,
                    range,
                };
                let _ = mapping.push(map);
            }
        }
    });

    // let result = mappings.iter().fold((0, mappings[0].clone()), |acc, mapping| {
    //     let index = acc.0;
    //     let this = acc.1;
    //     match index {
    //         0 => { (1, this) }
    //         _ => {
    //             let next = mapping;
    //
    //             let new_ranges = this.iter().map(|mapping| {
    //                 let a = next.iter().map(|next_mapping| {
    //                     // generate new ranges from comparision
    //                     let m = Mapping { source: 1, destination: 1, range: 1 };
    //                     m
    //                 }).collect::<Vec<Mapping>>();
    //                 a
    //             }).collect::<Vec<_>>().into_iter().flatten().collect::<Vec<Mapping>>();
    //             (index.clone() + 1, new_ranges)
    //         }
    //     }
    // }).1.into_iter().min();

    // println!("{:?}", result);
    let m2 = mappings.clone();
    let matches1 = merge_mappings(m2.get(0).unwrap(), m2.get(1).unwrap());
    let matches2 = merge_mappings(m2.get(2).unwrap(), m2.get(3).unwrap());
    let matches3 = merge_mappings(m2.get(4).unwrap(), m2.get(5).unwrap());
    let matches1 = merge_mappings(&matches1,&matches2);
    let matches2 = merge_mappings(&matches3,m2.get(6).unwrap());
    let matches = merge_mappings(&matches1, &matches2);
    println!("{:?}", matches);
    Ok(0)
}

fn merge_mappings(a: &Vec<Mapping>, b: &Vec<Mapping>) -> Vec<Mapping> {
    a.iter().map(|a_map| {
        let b_matches = b.iter().filter_map(|b_map| {
            match b_map {
                x if (a_map.source >= b_map.source && a_map.source <= b_map.source + b_map.range) || (
                    a_map.source + a_map.range >= b_map.source && a_map.source + a_map.range <= b_map.source + b_map.range
                ) => {
                    //exact match
                    Some(Mapping {
                        source: a_map.source,
                        destination: x.destination,
                        range: a_map.range,
                    })
                }

                &_ => {
                    println!("NONE");
                    None
                }
            }
        }).collect_vec();
        dbg!(b_matches.clone());
        b_matches
    }).flatten().collect()
}

#[derive(Debug, Eq, Copy, Clone, PartialEq, Ord, PartialOrd)]
struct Mapping {
    source: u64,
    destination: u64,
    range: u64,
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
