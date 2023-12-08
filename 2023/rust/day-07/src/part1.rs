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
    let hands: Vec<_> = _input.lines().map(|line| {
        let mut split = line.split_ascii_whitespace();
        let cards = split.next().expect("Should be a set of cards");
        let bid = split.next().expect("Should be a bid string").parse::<u64>().expect("Bid should be a valid number");
        Hand {
            cards: string_to_cards(cards),
            bid,
        }
    }).collect();

    let hands = sort(hands);

    // println!("{:?}", hands);
    let score = hands.iter().enumerate().map(|(i, hand)| {
        hand.bid * (i + 1) as u64
    }).sum();
    Ok(score)
}

fn sort(hands: Vec<Hand>) -> Vec<Hand> {
    hands.into_iter().sorted_by(|a, b| {
        let ranka = rank(&a.cards);
        let rankb = rank(&b.cards);
        if ranka > rankb { return Ordering::Greater; }
        if ranka < rankb { return Ordering::Less; }
        for i in 0..5 {
            let carda = a.cards.get(i).unwrap();
            let cardb = b.cards.get(i).unwrap();
            if carda > cardb { return Ordering::Greater; }
            if cardb > carda { return Ordering::Less; }
        }
        Ordering::Equal
    }).collect()
}

fn rank(cards: &[PlayingCard]) -> Type {
    let freq = cards.iter().fold(BTreeMap::new(), |mut map, card| {
        map.entry(card).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    let freqs = freq.values().collect::<Vec<_>>();
    match freqs {
        x if x.contains(&&5) => Kind(6),
        x if x.contains(&&4) => Kind(5),
        x if x.contains(&&3) && x.contains(&&2) => Kind(4),
        x if x.contains(&&3) => Kind(3),
        x if x.iter().filter(|x| ***x == 2).collect::<Vec<_>>().len() == 2 => Kind(2),
        x if x.contains(&&2) => Kind(1),
        _ => Kind(0)
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cards.cmp(&other.cards)
    }
}

impl Eq for Hand {}

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
