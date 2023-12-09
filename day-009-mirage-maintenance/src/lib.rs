use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sequence(Vec<i32>);

fn parse_sequence(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, parse_sequence)(input)
}

pub fn process(sequence: &[i32]) -> Vec<i32> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

pub fn extrapolate(sequence: &[i32], diffs: Point2D<i64>) -> Point2D<i64> {
    (
        sequence[0] as i64 - diffs.x,
        sequence[sequence.len() - 1] as i64 + diffs.y,
    )
        .into()
}

pub fn extrapolate_sequence(sequence: &[i32]) -> Point2D<i64> {
    let new = process(sequence);

    if new.iter().all(|v| *v == 0) {
        extrapolate(sequence, Point2D::default())
    } else {
        let e = extrapolate_sequence(&new);
        extrapolate(sequence, e)
    }
}

pub fn extrapolate_all(sequences: &[Vec<i32>]) -> Point2D<i64> {
    sequences
        .iter()
        .map(|s| extrapolate_sequence(s))
        .sum()
}

#[derive(Debug, Clone)]
pub struct MirageMaintenance {
    right: i64,
    left: i64,
}

impl FromStr for MirageMaintenance {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, sequences) = parse_sequences(s).map_err(|e| e.to_owned())?;

        let ans = extrapolate_all(&sequences);

        Ok(Self {
            left: ans.x,
            right: ans.y,
        })
    }
}

impl Problem for MirageMaintenance {
    const DAY: usize = 9;
    const TITLE: &'static str = "mirage maintenance";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.right)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.left)
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
        let solution = MirageMaintenance::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1969958987, 1068));
    }

    #[test]
    fn example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let solution = MirageMaintenance::solve(input).unwrap();
        assert_eq!(solution, Solution::new(114, 2));
    }
}
