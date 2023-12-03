use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::BoundedCardinalNeighbors, geometry::Location};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct GearRatios {
    part_total: u32,
    gear_total: u32,
}

impl GearRatios {
    // this is wonky, but there was more variance in this "common" functionality
    // than I originally thought.
    fn extract_numbers<I>(
        chars: &Grid<char>,
        iter: I,
        part_total: &mut u32,
        gear_total: &mut u32,
        processed: &mut FxHashSet<Location>,
        is_star: bool,
    ) where
        I: Iterator<Item = (Location, char)>,
    {
        let mut count = 0;
        let mut sub_prod = 1;
        'outer: for (can, ch) in iter {
            if processed.contains(&can) {
                continue;
            }

            processed.insert(can);

            let mut digits: u32 = 1;
            // we know this is safe
            let mut number: u32 = ch.to_digit(10).unwrap();

            // walk to the west
            let mut working = can;
            while let Some(west) = working.west() {
                if let Some(v) = chars.get(&west) {
                    if processed.contains(&west) {
                        continue 'outer;
                    }
                    if v.is_ascii_digit() {
                        working = west;
                        number += v.to_digit(10).unwrap() * 10_u32.pow(digits);
                        digits += 1;
                        processed.insert(west);
                        continue;
                    }
                }
                break;
            }

            // walk to the east
            let mut working = can;
            while let Some(east) = working.east() {
                if let Some(v) = chars.get(&east) {
                    if processed.contains(&east) {
                        continue 'outer;
                    }
                    if v.is_ascii_digit() {
                        working = east;
                        number = number * 10 + v.to_digit(10).unwrap();
                        processed.insert(east);
                        continue;
                    }
                }
                break;
            }

            *part_total += number;
            sub_prod *= number;
            count += 1;
        }

        if is_star && count == 2 {
            *gear_total += sub_prod;
        }
    }
}

impl FromStr for GearRatios {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Grid<char> = Grid::from_str(s)?;

        let mut part_total = 0;
        let mut gear_total = 0;
        let mut processed = FxHashSet::default();

        for row in 0..chars.height() {
            for col in 0..chars.width() {
                let loc = Location::new(row, col);
                let s = chars.locations[row][col];
                if !(s.is_ascii_digit() || s == '.') {
                    Self::extract_numbers(
                        &chars,
                        chars
                            .neighbors(&loc)
                            .filter(|(_, _, ch)| ch.is_ascii_digit())
                            .map(|(_, n, ch)| (n, *ch)),
                        &mut part_total,
                        &mut gear_total,
                        &mut processed,
                        s == '*',
                    );
                }
            }
        }

        Ok(Self {
            part_total,
            gear_total,
        })
    }
}

impl Problem for GearRatios {
    const DAY: usize = 3;
    const TITLE: &'static str = "gear ratios";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part_total)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.gear_total)
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
        let solution = GearRatios::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(526404, 84399773));
    }

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let solution = GearRatios::solve(input).unwrap();
        assert_eq!(solution, Solution::new(4361, 467835));
    }
}
