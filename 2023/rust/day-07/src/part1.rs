use std::cmp::Ordering;
use std::collections::BTreeMap;
use itertools::Itertools;
use crate::custom_error::AocError;
use crate::part1::PlayingCard::Card;
use crate::part1::Type::Kind;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<u64, AocError> {
    let mut hands: Vec<_> = _input.lines().map(|line| {
        let mut split = line.split_ascii_whitespace();
        let cards = split.next().expect("Should be a set of cards");
        let bid = split.next().expect("Should be a bid string").parse::<u64>().expect("Bid should be a valid number");
        Hand {
            cards: string_to_cards(cards),
            bid: bid,
        }
    }).collect();



    println!("{:?}", hands);
    let score = hands.iter().map(|hand| {

    }).sum();
    Ok(score)
}

fn sort(hands: Vec<Hand>) -> Vec<Hand> {
    let sorted = hands.iter().sorted_by(|a,b|{
        if rank(&a.cards) > rank(&b.cards) {return Ordering::Greater}
        if rank(&a.cards) < rank(&b.cards) {return Ordering::Less}
        return Ordering::Equal
    });
}

fn rank(cards: &Vec<PlayingCard>) -> Type {
    let freq = cards.iter().fold(BTreeMap::new(), |mut map, card| {
        map.entry(card).and_modify(|frq|*frq+=1).or_insert(1);
        map
    });
    println!("{:?}", freq);
    Kind(5)
}

enum Type {
    Kind(u8)
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
enum PlayingCard {
    Card(u8)
}

#[derive(Debug)]
struct Hand {
    cards: Vec<PlayingCard>,
    bid: u64,
}


impl Eq for Hand {}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cards.cmp(&other.cards)
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cards.partial_cmp(&other.cards)
    }
}


fn string_to_cards(string: &str) -> Vec<PlayingCard> {
    string.chars().map(|ch| {
        match ch {
            'A' => Card(15),
            'K' => Card(14),
            'Q' => Card(13),
            'J' => Card(12),
            'T' => Card(11),
            '9' => Card(10),
            '8' => Card(9),
            '7' => Card(8),
            '6' => Card(7),
            '5' => Card(6),
            '4' => Card(5),
            '3' => Card(4),
            '2' => Card(3),
            _ => panic!("Unknown character {ch}")
        }
    }).collect::<Vec<_>>()
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
