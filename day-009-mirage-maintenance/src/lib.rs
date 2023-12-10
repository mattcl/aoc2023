use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::PascalsTriangle;
use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn parse_sequence(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(complete::char(' '), complete::i32)(input)
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, parse_sequence)(input)
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
        let max_len = sequences.iter().map(|s| s.len()).max().unwrap_or_default();
        let triangle: PascalsTriangle<i64> = PascalsTriangle::new(max_len + 1);

        let mut right = 0;
        let mut left = 0;

        for seq in sequences {
            let row = seq.len();

            for (col, v) in seq.into_iter().enumerate() {
                let v = v as i64;

                if (row - col) % 2 == 0 {
                    right -= triangle[row][col] * v;
                } else {
                    right += triangle[row][col] * v;
                }

                if col % 2 == 0 {
                    left += triangle[row][col + 1] * v;
                } else {
                    left -= triangle[row][col + 1] * v;
                }
            }
        }

        Ok(Self { left, right })
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
