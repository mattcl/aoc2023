use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space0},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

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

fn parse_card(input: &str) -> IResult<&str, u32> {
    let (input, (winning, numbers)) = separated_pair(
        preceded(
            delimited(tag("Card"), preceded(space0, complete::u32), tag(":")),
            parse_map,
        ),
        tag(" |"),
        parse_map,
    )(input)?;

    let winning_count = (winning & numbers).count_ones();

    Ok((input, winning_count))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(newline, parse_card)(input)
}

#[derive(Debug, Clone)]
pub struct Scratchcards {
    p1: u32,
    p2: u32,
}

impl FromStr for Scratchcards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, cards) = parse_cards(s).map_err(|e| e.to_owned())?;

        let mut counts = vec![1_u32; cards.len()];

        for (idx, winning_count) in cards.iter().enumerate() {
            for c_idx in (idx + 1)..(idx + *winning_count as usize + 1) {
                counts[c_idx] += counts[idx];
            }
        }

        let p1 = cards
            .into_iter()
            // we can just do this with shifts
            .map(|worth| (1 << worth) >> 1)
            .sum();
        let p2 = counts.iter().sum();

        Ok(Self { p1, p2 })
    }
}

impl Problem for Scratchcards {
    const DAY: usize = 4;
    const TITLE: &'static str = "scratchcards";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
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
