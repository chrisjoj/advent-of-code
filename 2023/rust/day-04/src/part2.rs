use itertools::Itertools;
use crate::custom_error::AocError;
use std::str;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<i32, AocError> {
    let mut cards: Vec<i32> = Vec::from([1; 206]);
    cards[0] = 0;
    let mut card_no = 0;
    _input.lines().for_each(
        |line| {
            let mut a = line.split('|');
            let b = a.next().unwrap().split(':');
            // let card_no = b.next().unwrap();
            // let card_no = card_no[5..].trim().parse::<i32>().expect("should be a card number");#
            card_no += 1;
            let winning_nos = b.last().unwrap();
            let my_nos = a.last().unwrap();

            let winners: Vec<i32> = winning_nos.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_vec();

            let count = my_nos.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).fold(0, |acc, number| {
                match winners.contains(&number) {
                    true => {
                        acc + 1
                    }
                    false => { acc }
                }
            });

            match count {
                0 => {}
                _ => {
                    (0..count).for_each(|i| {
                        let future_card_count = cards.get((card_no + i + 1) as usize).unwrap();
                        let current_card_count = cards.get(card_no as usize).unwrap();
                        cards[(card_no + i + 1) as usize] = future_card_count + current_card_count;
                    });
                }
            }


            // score + count
        },
    );

    let tally = cards.iter().sum();
    Ok(tally)
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
        assert_eq!(30, process(input)?);
        Ok(())
    }
}
