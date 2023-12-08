use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::iter::Map;
use itertools::Itertools;
use crate::custom_error::AocError;
use crate::part2::PlayingCard::Card;
use crate::part2::Type::Kind;

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
        // println!("{:?}", hand.clone().bid);
        hand.bid * (i + 1) as u64
    }).sum();

    // match score {
    //     244848487 => Ok(score),
    //     _ => panic!("BAH! {score}")
    // }
    Ok(score)
}

fn sort(hands: Vec<Hand>) -> Vec<Hand> {
    hands.into_iter().sorted_by(|a, b| {
        let ranka = rank(&a.cards);
        let rankb = rank(&b.cards);
        for i in 0..5 {
            let carda = a.cards.get(i).unwrap();
            let cardb = b.cards.get(i).unwrap();
            if carda > cardb { return Ordering::Greater; }
            if carda < cardb { return Ordering::Less; }
        }
        Ordering::Equal
    }).collect()
}

fn best_card(freq: BTreeMap<&PlayingCard, i32>) -> &PlayingCard {
    freq.iter().sorted_by(|a, b| {
        let carda = a.0;
        let freqa = a.1;

        let cardb = b.0;
        let freqb = b.1;

        if freqa > freqb { return Ordering::Greater; }
        if freqb > freqa { return Ordering::Less; }
        if carda > cardb { return Ordering::Greater; }
        if cardb > carda { return Ordering::Less; }
        Ordering::Equal
    }).max().expect("Should be a sorted list of card frequency & strength").0
}

fn rank(cards: &[PlayingCard]) -> Type {
    // dbg!(cards.clone());
    let mut freq = cards.iter().fold(BTreeMap::new(), |mut map, card| {
        map.entry(card).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let joker_count = *freq.get(&Card(2)).or(Some(&0)).expect("Joker count should be found");
    let freqs = match joker_count {
        0 | 5 => {
            freq.values().copied().collect::<Vec<_>>()
        }
        _ => {
            freq.remove(&Card(2));
            let mut freqs = freq.values().copied().sorted().rev().collect::<Vec<_>>();
            let top_count = freqs[0] + joker_count;
            freqs[0] = top_count;
            freqs
        }
    };

    // dbg!(freq.clone());
    // dbg!(freqs.clone());
    match freqs {
        x if x.contains(&5) => Kind(6),
        x if x.contains(&4) => Kind(5),
        x if x.contains(&3) && x.contains(&2) => Kind(4),
        x if x.contains(&3) => Kind(3),
        x if x.iter().filter(|x| **x == 2).collect::<Vec<_>>().len() == 2 => Kind(2),
        x if x.contains(&2) => Kind(1),
        _ => Kind(0)
    }
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug)]
enum Type {
    Kind(u8)
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug, Clone)]
enum PlayingCard {
    Card(u8)
}

#[derive(Debug, Clone)]
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
            'A' => Card(14),
            'K' => Card(13),
            'Q' => Card(12),
            'T' => Card(11),
            '9' => Card(10),
            '8' => Card(9),
            '7' => Card(8),
            '6' => Card(7),
            '5' => Card(6),
            '4' => Card(5),
            '3' => Card(4),
            '2' => Card(3),
            'J' => Card(2),
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
        assert_eq!(5905, process(input)?);
        Ok(())
    }

    #[test]
    fn test_card_ordering() -> miette::Result<()> {
        let carda = string_to_cards("AA9AA");
        let cardb = string_to_cards("AATAJ");
        let cardc = string_to_cards("AATAA");
        let cards = vec!(Hand { cards: carda.clone(), bid: 0 }, Hand { cards: cardb.clone(), bid: 0 }, Hand { cards: cardc.clone(), bid: 0 });
        assert_eq!(true, carda < cardb);
        assert_eq!(true, cardb < cardc);
        assert_eq!(true, carda < cardc);
        let cards = sort(cards);
        assert_eq!(vec!(Hand { cards: cardc.clone(), bid: 0 }, Hand { cards: cardb.clone(), bid: 0 }, Hand { cards: carda.clone(), bid: 0 }), cards);
        Ok(())
    }
}
