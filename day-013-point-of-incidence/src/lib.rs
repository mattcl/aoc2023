use std::{fmt::Display, str::FromStr};

use aoc_plumbing::Problem;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitMirror {
    horizontal: Vec<u32>,
    vertical: Vec<u32>,
    width: usize,
    height: usize,
}

impl FromStr for BitMirror {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|l| l.len()).unwrap_or_default();

        let mut mirror = Self {
            vertical: vec![0; width],
            width,
            ..Default::default()
        };

        for (row, line) in s.lines().enumerate() {
            mirror
                .horizontal
                .push(line.chars().enumerate().fold(0, |acc, (col, ch)| {
                    if ch == '#' {
                        mirror.vertical[col] |= 1 << row;
                        acc | (1 << col)
                    } else {
                        acc
                    }
                }));
        }

        mirror.height = mirror.horizontal.len();

        Ok(mirror)
    }
}

impl Display for BitMirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.horizontal.iter() {
            writeln!(f, "{:018b}", row)?;
        }
        Ok(())
    }
}

// 987654321
// 011001101
// 010110100
// 100000011
// 100000011
// 010110100
// 011001100
// 010110101

impl BitMirror {
    pub fn reflect_horizontal(&self) -> Option<usize> {
        'outer: for i in 1..self.width {
            let limit = self.width - i;
            let adjust = 32 - limit.min(i);
            let mask = u32::MAX >> adjust;
            let shift = if limit < i { i - limit } else { 0 };
            for row in self.horizontal.iter() {
                let reversed = (row >> i).reverse_bits() >> adjust;
                let masked = (row >> shift) & mask;
                if masked != reversed {
                    continue 'outer;
                }
            }

            return Some(i);
        }

        None
    }

    pub fn reflect_vertical(&self) -> Option<usize> {
        'outer: for i in 1..self.height {
            let limit = self.height - i;
            let adjust = 32 - limit.min(i);
            let mask = u32::MAX >> adjust;
            let shift = if limit < i { i - limit } else { 0 };
            for col in self.vertical.iter() {
                let reversed = (col >> i).reverse_bits() >> adjust;
                let masked = (col >> shift) & mask;
                if masked != reversed {
                    continue 'outer;
                }
            }

            return Some(i * 100);
        }

        None
    }

    pub fn reflect_horizontal_one_off(&self) -> Option<usize> {
        'outer: for i in 1..self.width {
            let mut one_count = 0;
            let limit = self.width - i;
            let adjust = 32 - limit.min(i);
            let mask = u32::MAX >> adjust;
            let shift = if limit < i { i - limit } else { 0 };
            for row in self.horizontal.iter() {
                let reversed = (row >> i).reverse_bits() >> adjust;
                let masked = (row >> shift) & mask;
                one_count += (masked ^ reversed).count_ones();

                if one_count > 1 {
                    continue 'outer;
                }
            }

            if one_count == 1 {
                return Some(i);
            }
        }

        None
    }

    pub fn reflect_vertical_one_off(&self) -> Option<usize> {
        'outer: for i in 1..self.height {
            let mut one_count = 0;
            let limit = self.height - i;
            let adjust = 32 - limit.min(i);
            let mask = u32::MAX >> adjust;
            let shift = if limit < i { i - limit } else { 0 };
            for col in self.vertical.iter() {
                let reversed = (col >> i).reverse_bits() >> adjust;
                let masked = (col >> shift) & mask;
                one_count += (masked ^ reversed).count_ones();

                if one_count > 1 {
                    continue 'outer;
                }
            }

            if one_count == 1 {
                return Some(i * 100);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct PointOfIncidence {
    bit_mirrors: Vec<BitMirror>,
}

impl PointOfIncidence {
    pub fn summarize(&self) -> usize {
        self.bit_mirrors
            .iter()
            .map(|m| m.reflect_vertical().unwrap_or(0) + m.reflect_horizontal().unwrap_or(0))
            .sum()
    }

    pub fn fix_smudge(&self) -> usize {
        self.bit_mirrors
            .iter()
            .map(|m| {
                m.reflect_vertical_one_off().unwrap_or(0)
                    + m.reflect_horizontal_one_off().unwrap_or(0)
            })
            .sum()
    }
}

impl FromStr for PointOfIncidence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bit_mirrors = s
            .trim()
            .split("\n\n")
            .map(BitMirror::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { bit_mirrors })
    }
}

impl Problem for PointOfIncidence {
    const DAY: usize = 13;
    const TITLE: &'static str = "point of incidence";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.summarize())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.fix_smudge())
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
        let solution = PointOfIncidence::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(31739, 31539));
    }

    #[test]
    fn example() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let solution = PointOfIncidence::solve(input).unwrap();
        assert_eq!(solution, Solution::new(405, 400));
    }
}
