use crate::custom_error::AocError;
use crate::part1::Card::Card;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u64, AocError> {
    let mut hands:Vec<_> = _input.lines().map(|line|{
        let mut split = line.split_ascii_whitespace();
        let cards = split.next().expect("Should be a set of cards");
        let bid = split.next().expect("Should be a bid string").parse::<u64>().expect("Bid should be a valid number");
        Hand{
            cards:
            bid:bid
        }
    }).collect();

    hands.sort();
    let score = hands.map(|hand|{

    }).sum();


    Ok(score)
}

enum Card {
    Card(u8)
}

struct Hand {
    cards: [Card;5],
    bid: u64
}

fn string_to_cards(string :&str) -> Vec<Card> {
    string.chars().map(|ch|{
        match ch {
            'A' => Card(15),
            'K' => Card(14)

        }
    }) .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
