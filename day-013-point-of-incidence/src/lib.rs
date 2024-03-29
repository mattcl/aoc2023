use std::{fmt::Display, str::FromStr};

use aoc_std::geometry::Point2D;
use rayon::prelude::*;

use aoc_plumbing::Problem;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitMirror {
    horizontal: Vec<u32>,
    width: usize,
    height: usize,
}

impl FromStr for BitMirror {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|l| l.len()).unwrap_or_default();

        let mut mirror = Self {
            width,
            ..Default::default()
        };

        for line in s.lines() {
            mirror
                .horizontal
                .push(line.chars().enumerate().fold(0, |acc, (col, ch)| {
                    if ch == '#' {
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
        'outer: for i in 0..(self.height - 1) {
            let limit = self.height - i - 2;
            // expand outward
            for delta in 0..=i.min(limit) {
                if self.horizontal[i - delta] != self.horizontal[i + 1 + delta] {
                    continue 'outer;
                }
            }

            return Some((i + 1) * 100);
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
        'outer: for i in 0..(self.height - 1) {
            let mut one_count = 0;
            let limit = self.height - i - 2;
            // expand outward
            for delta in 0..=i.min(limit) {
                one_count +=
                    (self.horizontal[i - delta] ^ self.horizontal[i + 1 + delta]).count_ones();

                if one_count > 1 {
                    continue 'outer;
                }
            }

            if one_count == 1 {
                return Some((i + 1) * 100);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct PointOfIncidence {
    p1: usize,
    p2: usize,
}

impl FromStr for PointOfIncidence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s.trim().split("\n\n").collect::<Vec<_>>();
        let sums: Point2D<usize> = groups
            .par_iter()
            .filter_map(|group| {
                BitMirror::from_str(group).ok().map(|mirror| {
                    (
                        // I guess we're making the assumption that only
                        // one line of symmetry will be found per input,
                        // which seems to be true?
                        mirror
                            .reflect_horizontal()
                            .or_else(|| mirror.reflect_vertical())
                            .unwrap_or(0),
                        mirror
                            .reflect_horizontal_one_off()
                            .or_else(|| mirror.reflect_vertical_one_off())
                            .unwrap_or(0),
                    )
                        .into()
                })
            })
            .sum();
        Ok(Self {
            p1: sums.x,
            p2: sums.y,
        })
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
