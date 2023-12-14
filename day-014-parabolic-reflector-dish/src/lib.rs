use std::{fmt::Display, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::directions::Cardinal;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct Dish {
    rounds: Vec<u128>,
    cubes: Vec<u128>,
    width: usize,
}

impl Default for Dish {
    fn default() -> Self {
        Self {
            rounds: Vec::with_capacity(100),
            cubes: Vec::with_capacity(100),
            width: 0,
        }
    }
}

impl Dish {
    pub fn tilt(&mut self, dir: &Cardinal) {
        match dir {
            Cardinal::North => {
                for c in 0..self.width {
                    let mask = 1 << c;
                    let mut target = 0;
                    for r in 0..self.rounds.len() {
                        if self.cubes[r] & mask != 0 {
                            target = r + 1;
                        } else if self.rounds[r] & mask != 0 {
                            // remove this round from this row
                            self.rounds[r] &= !mask;
                            self.rounds[target] |= mask;
                            target += 1;
                        }
                    }
                }
            }
            Cardinal::West => {
                for r in 0..self.rounds.len() {
                    let mut target = self.width - 1;
                    for c in (0..self.width).rev() {
                        let mask = 1 << c;
                        if self.cubes[r] & mask != 0 && c != 0 {
                            target = c - 1;
                        } else if self.rounds[r] & mask != 0 {
                            // remove this round from this row
                            self.rounds[r] &= !mask;
                            self.rounds[r] |= 1 << target;
                            if target != 0 {
                                target -= 1;
                            }
                        }
                    }
                }
            }
            Cardinal::South => {
                for c in 0..self.width {
                    let mask = 1 << c;
                    let mut target = self.rounds.len() - 1;
                    for r in (0..self.rounds.len()).rev() {
                        if self.cubes[r] & mask != 0 && r != 0 {
                            target = r - 1;
                        } else if self.rounds[r] & mask != 0 {
                            // remove this round from this row
                            self.rounds[r] &= !mask;
                            self.rounds[target] |= mask;
                            if target != 0 {
                                target -= 1;
                            }
                        }
                    }
                }
            }
            Cardinal::East => {
                for r in 0..self.rounds.len() {
                    let mut target = 0;
                    for c in 0..self.width {
                        if self.rounds[r] >> c == 0 {
                            break;
                        }
                        let mask = 1 << c;
                        if self.cubes[r] & mask != 0 {
                            target = c + 1;
                        } else if self.rounds[r] & mask != 0 {
                            // remove this round from this row
                            self.rounds[r] &= !mask;
                            self.rounds[r] |= 1 << target;
                            target += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn cycle(&mut self, count: usize) -> u32 {
        let mut cache: FxHashMap<u128, usize> = FxHashMap::default();
        let mut loads: Vec<u32> = Vec::default();
        for cycle_idx in 0..count {
            self.tilt(&Cardinal::North);
            self.tilt(&Cardinal::West);
            self.tilt(&Cardinal::South);
            self.tilt(&Cardinal::East);
            let load = self.total_load();

            // make a key with the last 8 loads
            if cycle_idx > 4 {
                let key: u128 = loads[(loads.len() - 5)..]
                    .iter()
                    .fold(0, |acc, v| (acc << 32 | *v as u128));
                // let key_b: u128 = loads[(loads.len() - 9)..(loads.len() - 4)]
                //     .iter()
                //     .fold(0, |acc, v| (acc << 32 | *v as u128));
                // let key = (key_a, key_b);

                let e = cache.entry(key).or_insert(cycle_idx);

                if *e != cycle_idx {
                    let period = cycle_idx - *e;

                    if count % period == cycle_idx % period {
                        let rem = (count - cycle_idx) % period;
                        // we need to advance by rem in loads from the last index - 1
                        return loads[*e + rem - 1];
                    }
                }
            }

            loads.push(load);
        }

        self.total_load()
    }

    pub fn total_load(&self) -> u32 {
        let mut total = 0;
        for r in 0..self.rounds.len() {
            total += self.rounds[r].count_ones() as u32 * (self.rounds.len() - r) as u32;
        }
        total
    }
}

impl Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rounds.iter() {
            writeln!(f, "{:020b}", row)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ParabolicReflectorDish {
    dish: Dish,
}

impl FromStr for ParabolicReflectorDish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dish = Dish::default();

        dish.width = s.lines().next().unwrap().len();

        for line in s.trim().lines() {
            let mut cubes = 0;
            let mut rounds = 0;
            for (c, ch) in line.chars().enumerate() {
                match ch {
                    '#' => cubes |= 1 << dish.width - c - 1,
                    'O' => rounds |= 1 << dish.width - c - 1,
                    _ => {}
                }
            }
            dish.cubes.push(cubes);
            dish.rounds.push(rounds);
        }

        Ok(Self { dish })
    }
}

impl Problem for ParabolicReflectorDish {
    const DAY: usize = 14;
    const TITLE: &'static str = "parabolic reflector dish";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut dish = self.dish.clone();
        dish.tilt(&Cardinal::North);
        Ok(dish.total_load())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.dish.cycle(1_000_000_000))
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
        let solution = ParabolicReflectorDish::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(105982, 85175));
    }

    #[test]
    fn example() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let solution = ParabolicReflectorDish::solve(input).unwrap();
        assert_eq!(solution, Solution::new(136, 64));
    }
}
