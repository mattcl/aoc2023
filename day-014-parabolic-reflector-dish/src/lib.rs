use std::{fmt::Display, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{directions::Cardinal, geometry::{Interval, Point2D}, collections::Grid};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Dish2 {
    rounds: Vec<Point2D<i8>>,
    row_intervals: Vec<Vec<Interval<i8>>>,
    col_intervals: Vec<Vec<Interval<i8>>>,
    rounds_in_rows: Vec<Vec<usize>>,
    rounds_in_cols: Vec<Vec<usize>>,
}

impl Dish2 {
    fn total_load(&self) -> u32 {
        let height = self.row_intervals.len() as u32;
        self.rounds.iter().map(|r| height - r.x as u32).sum()
    }

    pub fn cycle(&mut self, count: usize) -> u32 {
        let mut cache: FxHashMap<u128, usize> = FxHashMap::default();
        let mut loads: Vec<u32> = Vec::with_capacity(500);
        let mut buckets = vec![vec![]; 100];
        for cycle_idx in 0..count {
            self.tilt_north(&mut buckets);
            self.tilt_west(&mut buckets);
            self.tilt_south(&mut buckets);
            self.tilt_east(&mut buckets);
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

                    dbg!("potential");

                    if count % period == cycle_idx % period {
                        dbg!(&self.row_intervals);
                        dbg!(cycle_idx);
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

    fn tilt_north(&mut self, buckets: &mut Vec<Vec<usize>>) {
        for (col, intervals) in self.col_intervals.iter().enumerate() {
            for round_idx in self.rounds_in_cols[col].drain(..) {
                for (interval_idx, interval) in intervals.iter().enumerate() {
                    if interval.contains_value(self.rounds[round_idx].x) {
                        buckets[interval_idx].push(round_idx);
                    }
                }
            }

            for (interval_idx, bucket) in buckets.iter_mut().enumerate() {
                for (p, round_idx) in bucket.drain(..).enumerate() {
                    // move ourself (it doesn't matter the order)
                    self.rounds[round_idx].x = intervals[interval_idx].start + p as i8;
                    // prep for the next cycle
                    self.rounds_in_rows[self.rounds[round_idx].x as usize].push(round_idx);
                }
            }
        }
    }

    fn tilt_south(&mut self, buckets: &mut Vec<Vec<usize>>) {
        for (col, intervals) in self.col_intervals.iter().enumerate() {
            for round_idx in self.rounds_in_cols[col].drain(..) {
                for (interval_idx, interval) in intervals.iter().enumerate() {
                    if interval.contains_value(self.rounds[round_idx].x) {
                        buckets[interval_idx].push(round_idx);
                    }
                }
            }

            for (interval_idx, bucket) in buckets.iter_mut().enumerate() {
                for (p, round_idx) in bucket.drain(..).enumerate() {
                    // move ourself (it doesn't matter the order)
                    self.rounds[round_idx].x = intervals[interval_idx].end - p as i8;
                    // prep for the next cycle
                    self.rounds_in_rows[self.rounds[round_idx].x as usize].push(round_idx);
                }
            }
        }
    }

    fn tilt_east(&mut self, buckets: &mut Vec<Vec<usize>>) {
        for (row, intervals) in self.row_intervals.iter().enumerate() {
            for round_idx in self.rounds_in_rows[row].drain(..) {
                for (interval_idx, interval) in intervals.iter().enumerate() {
                    if interval.contains_value(self.rounds[round_idx].y) {
                        buckets[interval_idx].push(round_idx);
                    }
                }
            }

            for (interval_idx, bucket) in buckets.iter_mut().enumerate() {
                for (p, round_idx) in bucket.drain(..).enumerate() {
                    // move ourself (it doesn't matter the order)
                    self.rounds[round_idx].y = intervals[interval_idx].end - p as i8;
                    // prep for the next cycle
                    self.rounds_in_cols[self.rounds[round_idx].y as usize].push(round_idx);
                }
            }
        }
    }

    fn tilt_west(&mut self, buckets: &mut Vec<Vec<usize>>) {
        for (row, intervals) in self.row_intervals.iter().enumerate() {
            for round_idx in self.rounds_in_rows[row].drain(..) {
                for (interval_idx, interval) in intervals.iter().enumerate() {
                    if interval.contains_value(self.rounds[round_idx].y) {
                        buckets[interval_idx].push(round_idx);
                    }
                }
            }

            for (interval_idx, bucket) in buckets.iter_mut().enumerate() {
                for (p, round_idx) in bucket.drain(..).enumerate() {
                    // move ourself (it doesn't matter the order)
                    self.rounds[round_idx].y = intervals[interval_idx].start + p as i8;
                    // prep for the next cycle
                    self.rounds_in_cols[self.rounds[round_idx].y as usize].push(round_idx);
                }
            }
        }
    }
}


impl FromStr for Dish2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<char> = Grid::from_str(s)?;
        let mut dish = Dish2 {
            rounds_in_rows: vec![vec![]; grid.height()],
            rounds_in_cols: vec![vec![]; grid.width()],
            ..Default::default()
        };

        let mut col_interval_markers = vec![vec![-1]; grid.width()];

        for row in 0..grid.height() {
            let mut interval_markers = vec![-1];
            for col in 0..grid.width() {
                match grid.locations[row][col] {
                    '#' => {
                        col_interval_markers[col].push(row as i8);
                        interval_markers.push(col as i8);
                    },
                    'O' => {
                        // we start always with NORTH, so we want cols first
                        // dish.rounds_in_rows[row].push(dish.rounds.len());
                        dish.rounds_in_cols[col].push(dish.rounds.len());
                        dish.rounds.push((row as i8, col as i8).into());
                    }
                    _ => {},
                }
            }
            interval_markers.push(grid.width() as i8);

            let mut intervals = Vec::with_capacity(interval_markers.len());
            for (start, end) in interval_markers.into_iter().tuple_windows() {
                if end - start < 2 {
                    continue;
                }
                intervals.push(Interval::new(start + 1, end - 1));
            }
            dish.row_intervals.push(intervals);
        }

        for mut col_markers in col_interval_markers {
            col_markers.push(grid.height() as i8);
            let mut intervals = Vec::with_capacity(col_markers.len());
            for (start, end) in col_markers.into_iter().tuple_windows() {
                if end - start < 2 {
                    continue;
                }
                intervals.push(Interval::new(start + 1, end - 1));
            }
            dish.col_intervals.push(intervals);
        }

        Ok(dish)
    }
}


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
    dish2: Dish2,
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

        let dish2 = Dish2::from_str(s)?;

        Ok(Self { dish, dish2 })
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
        let mut dish = self.dish2.clone();
        let mut buckets = vec![vec![]; 100];
        dish.tilt_north(&mut buckets);
        Ok(dish.total_load())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.dish2.cycle(1_000_000_000))
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
        assert!(false);
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
