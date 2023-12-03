use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::BoundedCardinalNeighbors, geometry::Location};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct GearRatios {
    chars: Grid<char>,
    symbol_map: FxHashMap<char, Vec<Vec<Location>>>,
}

impl GearRatios {
    // this is wonky, but there was more variance in this "common" functionality
    // than I originally thought.
    fn extract_numbers<'a, I>(
        &self,
        iter: I,
        total: &mut u32,
        target_count: u32,
        processed: &mut FxHashSet<Location>,
        sum: bool,
    ) -> bool
    where
        I: Iterator<Item = &'a Location>,
    {
        let mut count = 0;
        'outer: for can in iter {
            if processed.contains(can) {
                continue;
            }

            processed.insert(*can);

            let mut digits: u32 = 1;
            // we know this is safe
            let mut number: u32 = self.chars.get(can).unwrap().to_digit(10).unwrap();

            // walk to the west
            let mut working = *can;
            while let Some(west) = working.west() {
                if let Some(v) = self.chars.get(&west) {
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
            let mut working = *can;
            while let Some(east) = working.east() {
                if let Some(v) = self.chars.get(&east) {
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

            if sum {
                *total += number;
            } else {
                *total *= number;
            }
            count += 1;
            if target_count > 0 && count > target_count {
                return false;
            }
        }

        count == target_count
    }

    pub fn find_part_numbers(&self) -> u32 {
        let mut processed: FxHashSet<Location> = FxHashSet::default();

        let mut total_sum = 0;

        // we need to turn all the candidate locations into numbers
        for svs in self.symbol_map.values() {
            self.extract_numbers(
                svs.iter().flatten(),
                &mut total_sum,
                0,
                &mut processed,
                true,
            );
        }

        total_sum
    }

    pub fn find_gear_numbers(&self) -> u32 {
        let mut total_sum = 0;

        for gear_candidates in self.symbol_map.get(&'*').unwrap().iter() {
            let mut processed: FxHashSet<Location> = FxHashSet::default();
            let mut sub_product = 1;

            if self.extract_numbers(
                gear_candidates.iter(),
                &mut sub_product,
                2,
                &mut processed,
                false,
            ) {
                total_sum += sub_product
            }
        }

        total_sum
    }
}

impl FromStr for GearRatios {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Grid<char> = Grid::from_str(s)?;

        let mut symbol_map: FxHashMap<char, Vec<Vec<Location>>> = FxHashMap::default();

        for row in 0..chars.height() {
            for col in 0..chars.width() {
                let loc = Location::new(row, col);
                let s = chars.locations[row][col];
                if !(s.is_ascii_digit() || s == '.') {
                    let symbol_entry = symbol_map.entry(s).or_default();
                    let mut loc_entry = Vec::default();
                    for (_, n, ch) in chars.neighbors(&loc) {
                        if ch.is_ascii_digit() {
                            loc_entry.push(n);
                        }
                    }
                    symbol_entry.push(loc_entry);
                }
            }
        }

        Ok(Self { chars, symbol_map })
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
        Ok(self.find_part_numbers())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.find_gear_numbers())
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
