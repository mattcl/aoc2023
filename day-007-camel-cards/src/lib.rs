use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, multispace1},
    combinator::{self, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rustc_hash::FxHashMap;

const JOKER: u8 = 16;
const JOKER_MASK: u8 = !JOKER;

fn card_to_value(card: u8) -> u8 {
    match card {
        b'2' => 1,
        b'3' => 2,
        b'4' => 3,
        b'5' => 4,
        b'6' => 5,
        b'7' => 6,
        b'8' => 7,
        b'9' => 8,
        b'T' => 9,
        b'J' => JOKER,
        b'Q' => 32,
        b'K' => 33,
        b'A' => 34,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardSet {
    cards: [u8; 5],
}

impl FromStr for CardSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            bail!("Invalid length hand");
        }
        let bytes = s.as_bytes();
        Ok(Self {
            cards: [
                card_to_value(bytes[0]),
                card_to_value(bytes[1]),
                card_to_value(bytes[2]),
                card_to_value(bytes[3]),
                card_to_value(bytes[4]),
            ],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JokerCardSet {
    cards: [u8; 5],
}

impl From<&CardSet> for JokerCardSet {
    fn from(value: &CardSet) -> Self {
        Self {
            cards: [
                value.cards[0] & JOKER_MASK,
                value.cards[1] & JOKER_MASK,
                value.cards[2] & JOKER_MASK,
                value.cards[3] & JOKER_MASK,
                value.cards[4] & JOKER_MASK,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandKind {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    pub fn kinds_for_card_counts(cards: &CardSet) -> (Self, Self) {
        let mut counts: FxHashMap<u8, usize> = FxHashMap::default();
        let mut max = 0;
        for c in cards.cards.iter() {
            let e = counts.entry(*c).or_default();
            *e += 1;
            if *e > max {
                max = *e;
            }
        }
        let joker_count = counts.get(&JOKER).copied().unwrap_or_default();

        let len = counts.len();

        match (len, max) {
            (1, _) => (Self::FiveOfAKind, Self::FiveOfAKind),
            (5, _) => match joker_count {
                0 => (Self::HighCard, Self::HighCard),
                _ => (Self::HighCard, Self::OnePair),
            },
            (2, 4) => match joker_count {
                0 => (Self::FourOfAKind, Self::FourOfAKind),
                _ => (Self::FourOfAKind, Self::FiveOfAKind),
            },
            (2, _) => match joker_count {
                0 => (Self::FullHouse, Self::FullHouse),
                _ => (Self::FullHouse, Self::FiveOfAKind),
            },
            (_, 3) => match joker_count {
                0 => (Self::ThreeOfAKind, Self::ThreeOfAKind),
                _ => (Self::ThreeOfAKind, Self::FourOfAKind),
            },
            (4, _) => match joker_count {
                0 => (Self::OnePair, Self::OnePair),
                _ => (Self::OnePair, Self::ThreeOfAKind),
            },
            _ => match joker_count {
                1 => (Self::TwoPair, Self::FullHouse),
                2 => (Self::TwoPair, Self::FourOfAKind),
                _ => (Self::TwoPair, Self::TwoPair),
            }
        }

//         let joker_kind = if joker_count > 0 {
//             match kind {
//                 HandKind::FiveOfAKind => HandKind::FiveOfAKind
//             }
//         } else {
//             kind
//         };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hand {
    kind: HandKind,
    cards: CardSet,
    joker_kind: HandKind,
    joker_cards: JokerCardSet,
    bid: u64,
}

impl Hand {
    pub fn new(cards: CardSet, bid: u64) -> Self {
        let joker_cards = (&cards).into();

        let (kind, joker_kind) = HandKind::kinds_for_card_counts(&cards);
        Self {
            kind,
            cards,
            joker_kind,
            joker_cards,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind
            .cmp(&other.kind)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    combinator::map(
        separated_pair(
            map_res(alphanumeric1, CardSet::from_str),
            tag(" "),
            complete::u64,
        ),
        |(cards, bid)| Hand::new(cards, bid),
    )(input)
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, mut hands) = separated_list1(multispace1, parse_hand)(input)?;

    hands.sort();

    Ok((input, hands))
}

#[derive(Debug, Clone)]
pub struct CamelCards {
    hands: Vec<Hand>,
}

impl FromStr for CamelCards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, hands) = parse_hands(s).map_err(|e| e.to_owned())?;
        Ok(Self { hands })
    }
}

impl Problem for CamelCards {
    const DAY: usize = 7;
    const TITLE: &'static str = "camel cards";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.hands.sort_by(|a, b| {
            a.joker_kind
                .cmp(&b.joker_kind)
                .then_with(|| a.joker_cards.cmp(&b.joker_cards))
        });
        Ok(self
            .hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = CamelCards::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(250453939, 248652697));
    }

    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let solution = CamelCards::solve(input).unwrap();
        assert_eq!(solution, Solution::new(6440, 5905));
    }
}
