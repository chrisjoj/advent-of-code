use itertools::Itertools;
use crate::custom_error::AocError;
use std::str;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<i32, AocError> {
    let result = _input.lines().fold(
        0,
        |score, line| {
            let mut a = line.split('|');
            let b = a.next().unwrap().split(':');
            let winning_nos = b.last().unwrap();
            let my_nos = a.last().unwrap();

            let winners: Vec<i32> = winning_nos.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_vec();
            let count = my_nos.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).fold(0, |acc, number| {
                match winners.contains(&number) {
                    true => {
                        match acc {
                            0 => 1,
                            _ => acc * 2
                        }
                    }
                    false => { acc }
                }
            });
            score + count
        },
    );

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
