use std::str::FromStr;

use aoc_plumbing::Problem;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
pub struct WaitForIt {
    times: Vec<i64>,
    records: Vec<i64>,
}

impl WaitForIt {
    pub fn ways_to_beat(&self, time: i64, record: i64) -> i64 {
        let t = time as f64;
        let t2 = t * t;
        let r = record as f64;

        // solutions for (time - x) * x > record
        let lower_raw = 0.5 * (t - (t2 - 4.0 * r).sqrt());
        let upper_raw = 0.5 * (t + (t2 - 4.0 * r).sqrt());

        let mut lower = lower_raw.ceil() as i64;
        let mut upper = upper_raw.floor() as i64;

        // correct for weird errors with small numbers
        while (time - lower) * lower <= record {
            lower += 1;
        }

        while (time - upper) * upper <= record {
            upper -= 1;
        }

        upper - lower + 1
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(
        delimited(alpha1, tag(":"), space1),
        separated_list1(space1, complete::i64),
    )(input)
}

fn parse_data(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    separated_pair(parse_numbers, newline, parse_numbers)(input)
}

impl FromStr for WaitForIt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (times, records)) = parse_data(s).map_err(|e| e.to_owned())?;
        Ok(Self { times, records })
    }
}

impl Problem for WaitForIt {
    const DAY: usize = 6;
    const TITLE: &'static str = "wait for it";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok((0..self.times.len())
            .map(|idx| self.ways_to_beat(self.times[idx], self.records[idx]))
            .product())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let time: i64 = self.times.iter().join("").parse()?;
        let record: i64 = self.records.iter().join("").parse()?;
        Ok(self.ways_to_beat(time, record))
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
        let solution = WaitForIt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1312850, 36749103));
    }

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let solution = WaitForIt::solve(input).unwrap();
        assert_eq!(solution, Solution::new(288, 71503));
    }
}
