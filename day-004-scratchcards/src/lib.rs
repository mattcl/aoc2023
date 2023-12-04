use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space0},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Card {
    id: u32,
    winning_count: usize,
}

impl Card {
    pub fn worth(&self) -> u32 {
        if self.winning_count == 0 {
            return 0;
        }

        2_u32.pow(self.winning_count as u32 - 1)
    }
}

fn parse_map(input: &str) -> IResult<&str, u128> {
    fold_many1(
        preceded(space0, complete::u8),
        // the largest number we ever see is only two digits
        || 0_u128,
        |mut m, v| {
            m |= 1 << v;
            m
        },
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, ((id, winning), numbers)) = separated_pair(
        tuple((
            delimited(tag("Card"), preceded(space0, complete::u32), tag(":")),
            parse_map,
        )),
        tag(" |"),
        parse_map,
    )(input)?;

    let winning_count = (winning & numbers).count_ones() as usize;

    let card = Card { id, winning_count };

    Ok((input, card))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, parse_card)(input)
}

#[derive(Debug, Clone)]
pub struct Scratchcards {
    cards: Vec<Card>,
}

impl Scratchcards {
    pub fn recur(&self, idx: usize, memo: &mut FxHashMap<usize, u64>) -> u64 {
        if let Some(v) = memo.get(&idx) {
            return *v;
        }

        if idx >= self.cards.len() {
            return 0;
        }

        if self.cards[idx].winning_count == 0 {
            return 0;
        }

        let mut sum = self.cards[idx].winning_count as u64;

        for i in 0..self.cards[idx].winning_count {
            sum += self.recur(idx + i + 1, memo);
        }

        memo.insert(idx, sum);

        sum
    }
}

impl FromStr for Scratchcards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, cards) = parse_cards(s).map_err(|e| e.to_owned())?;
        Ok(Self { cards })
    }
}

impl Problem for Scratchcards {
    const DAY: usize = 4;
    const TITLE: &'static str = "scratchcards";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.cards.iter().map(|c| c.worth()).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut memo = FxHashMap::default();
        let mut sum = 0;
        for card in self.cards.iter() {
            sum += 1 + self.recur(card.id as usize - 1, &mut memo);
        }
        Ok(sum)
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
        let solution = Scratchcards::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(15268, 6283755));
    }

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let solution = Scratchcards::solve(input).unwrap();
        assert_eq!(solution, Solution::new(13, 30));
    }
}
