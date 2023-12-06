use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u64, AocError> {
    let races: Vec<Vec<u64>> = _input.lines().map(|line| {
        let l = line.split(':');
        l.last().unwrap().split_ascii_whitespace().map(|number| {
            number.parse::<u64>().expect("Should be a valid number")
        }).collect()
    }).collect();

    let race_times = races.get(0).expect("Should be a list of race length");
    let records = races.get(1).expect("Should have a list of racords.");

    let ways_to_win: u64 = race_times.iter().zip(records.into_iter()).map(|race| {
        let race_length = race.0;
        let record = race.1;

        (0..=*race_length).fold(0, |acc, charge_time| {
            match distance(*race_length, charge_time) > *record {
                true => { acc + 1 }
                false => { acc }
            }
        },
        )
    }).product();

    // dbg!(ways_to_win);
    Ok(ways_to_win)
}

fn distance(race_time: u64, charge: u64) -> u64 {
    // println!("distance: {},  time:{race_time?}, charge:{charge}",charge*(race_time-charge));
    charge * (race_time - charge)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      71530
Distance:  940200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
