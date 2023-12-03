use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::BoundedCardinalNeighbors, geometry::Location};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    value: u32,
    start: usize,
    end: usize,
}

impl Number {
    pub fn contains(&self, col: usize) -> bool {
        self.start <= col && self.end >= col
    }
}

#[derive(Debug, Clone)]
pub struct GearRatios {
    chars: Grid<char>,
}

impl GearRatios {
    // find all the symbols and then check all candidate spaces around them for
    // numbers
    pub fn find_numbers(&self) -> u32 {
        let mut candidates: FxHashSet<Location> = FxHashSet::default();

        for row in 0..self.chars.height() {
            for col in 0..self.chars.width() {
                let loc = Location::new(row, col);
                let s = self.chars.locations[row][col];
                if !(s.is_digit(10) || s == '.') {
                    for (_, n, ch) in self.chars.neighbors(&loc) {
                        if ch.is_digit(10) {
                            candidates.insert(n);
                        }
                    }
                }
            }
        }

        let mut processed: FxHashSet<Location> = FxHashSet::default();

        let mut total_sum = 0;

        // we need to turn all the candidate locations into numbers
        'outer: for can in candidates {
            if processed.contains(&can) {
                continue;
            }
            processed.insert(can);
            let mut number = VecDeque::default();

            // we know this is safe
            number.push_back(self.chars.get(&can).unwrap().to_digit(10).unwrap());

            let mut working = can;
            while let Some(west) = working.west() {
                if let Some(v) = self.chars.get(&west) {
                    if processed.contains(&west) {
                        continue 'outer;
                    }
                    if v.is_digit(10) {
                        working = west;
                        number.push_front(v.to_digit(10).unwrap());
                        processed.insert(west);
                        continue;
                    }
                }
                break;
            }

            let mut working = can;
            while let Some(east) = working.east() {
                if let Some(v) = self.chars.get(&east) {
                    if processed.contains(&east) {
                        continue 'outer;
                    }
                    if v.is_digit(10) {
                        working = east;
                        number.push_back(v.to_digit(10).unwrap());
                        processed.insert(east);
                        continue;
                    }
                }
                break;
            }

            // make the number from the end to the start
            let mut sum = 0;
            for i in 0..number.len() {
                sum = sum * 10 + number[i];
            }

            total_sum += sum;
        }

        total_sum
    }

    pub fn find_gears(&self) -> u32 {
        let mut candidates: FxHashMap<Location, Vec<Location>> = FxHashMap::default();

        for row in 0..self.chars.height() {
            for col in 0..self.chars.width() {
                let loc = Location::new(row, col);
                let s = self.chars.locations[row][col];
                if s == '*' {
                    for (_, n, ch) in self.chars.neighbors(&loc) {
                        if ch.is_digit(10) {
                            let e = candidates.entry(loc).or_default();
                            e.push(n);
                        }
                    }
                }
            }
        }

        let mut total_sum = 0;

        for (_gear, gear_candidates) in candidates {
            let mut processed: FxHashSet<Location> = FxHashSet::default();
            let mut nums = Vec::default();

            // we need to turn all the candidate locations into numbers
            'outer: for can in gear_candidates {
                if processed.contains(&can) {
                    continue;
                }
                processed.insert(can);
                let mut number = VecDeque::default();

                // we know this is safe
                number.push_back(self.chars.get(&can).unwrap().to_digit(10).unwrap());

                let mut working = can;
                while let Some(west) = working.west() {
                    if let Some(v) = self.chars.get(&west) {
                        if processed.contains(&west) {
                            continue 'outer;
                        }
                        if v.is_digit(10) {
                            working = west;
                            number.push_front(v.to_digit(10).unwrap());
                            processed.insert(west);
                            continue;
                        }
                    }
                    break;
                }

                let mut working = can;
                while let Some(east) = working.east() {
                    if let Some(v) = self.chars.get(&east) {
                        if processed.contains(&east) {
                            continue 'outer;
                        }
                        if v.is_digit(10) {
                            working = east;
                            number.push_back(v.to_digit(10).unwrap());
                            processed.insert(east);
                            continue;
                        }
                    }
                    break;
                }

                // make the number from the end to the start
                let mut sum: u32 = 0;
                for i in 0..number.len() {
                    sum = sum * 10 + number[i];
                }

                nums.push(sum);

                if nums.len() > 2 {
                    break;
                }
            }

            if nums.len() == 2 {
                total_sum += nums.into_iter().product::<u32>();
            }
        }

        total_sum
    }
}

impl FromStr for GearRatios {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            chars: Grid::from_str(s)?,
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
        Ok(self.find_numbers())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.find_gears())
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
