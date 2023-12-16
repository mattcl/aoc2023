use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, geometry::Interval};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone)]
pub struct Dish {
    row_intervals: Vec<Vec<Interval<i8>>>,
    col_intervals: Vec<Vec<Interval<i8>>>,
    rounds_in_rows: Vec<Vec<i8>>,
    rounds_in_cols: Vec<Vec<i8>>,
}

impl Dish {
    pub fn total_load_p1(&self) -> u32 {
        let height = self.rounds_in_rows.len();
        self.rounds_in_rows
            .iter()
            .enumerate()
            .map(|(i, r)| (r.len() * (height - i)) as u32)
            .sum()
    }

    pub fn cycle(&mut self, count: usize) -> u32 {
        let mut cache: FxHashMap<u128, usize> = FxHashMap::default();
        let mut loads: Vec<u32> = Vec::with_capacity(500);
        for cycle_idx in 0..count {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            let load = self.tilt_east();

            loads.push(load);

            // make a key with the last ~8~ 4 loads
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

                    let rem = (count - cycle_idx) % period;
                    // we need to advance by rem in loads from the last index - 1
                    return loads[*e + rem - 1];
                }
            }
        }

        // we should never make it here
        unreachable!("how did this happen?")
    }

    fn tilt_north(&mut self) {
        for (col, intervals) in self.col_intervals.iter().enumerate() {
            // we know the intervals are in sorted order
            let mut interval_idx = 0;
            let mut interval_insert_count = 0;
            // by the nature of how we insert things, these should always be
            // sorted (because the intervals are also in order)
            // self.rounds_in_cols[col].sort();

            for value in self.rounds_in_cols[col].drain(..) {
                while !intervals[interval_idx].contains_value(value) {
                    interval_idx += 1;
                    interval_insert_count = 0;
                }
                // start assigning values from the start of the interval and
                // working up
                let new_value = intervals[interval_idx].start + interval_insert_count;
                self.rounds_in_rows[new_value as usize].push(col as i8);
                interval_insert_count += 1;
            }
        }
    }

    fn tilt_south(&mut self) {
        for (col, intervals) in self.col_intervals.iter().enumerate() {
            let mut interval_idx = 0;
            let mut interval_insert_count = 0;

            for value in self.rounds_in_cols[col].drain(..) {
                while !intervals[interval_idx].contains_value(value) {
                    interval_idx += 1;
                    interval_insert_count = 0;
                }
                // start assigning values from the end of the interval and
                // working down
                let new_value = intervals[interval_idx].end - interval_insert_count;
                self.rounds_in_rows[new_value as usize].push(col as i8);
                interval_insert_count += 1;
            }
        }
    }

    fn tilt_west(&mut self) {
        for (row, intervals) in self.row_intervals.iter().enumerate() {
            let mut interval_idx = 0;
            let mut interval_insert_count = 0;

            for value in self.rounds_in_rows[row].drain(..) {
                while !intervals[interval_idx].contains_value(value) {
                    interval_idx += 1;
                    interval_insert_count = 0;
                }
                let new_value = intervals[interval_idx].start + interval_insert_count;
                self.rounds_in_cols[new_value as usize].push(row as i8);
                interval_insert_count += 1;
            }
        }
    }

    /// we can caluclate the north load after finishing the east tilt without
    /// having to iterate through _again._
    fn tilt_east(&mut self) -> u32 {
        let mut load = 0;
        let height = self.rounds_in_rows.len() as u32;
        for (row, intervals) in self.row_intervals.iter().enumerate() {
            let mut interval_idx = 0;
            let mut interval_insert_count = 0;

            for value in self.rounds_in_rows[row].drain(..) {
                while !intervals[interval_idx].contains_value(value) {
                    interval_idx += 1;
                    interval_insert_count = 0;
                }
                let new_value = intervals[interval_idx].end - interval_insert_count;
                load += height - row as u32;
                self.rounds_in_cols[new_value as usize].push(row as i8);
                interval_insert_count += 1;
            }
        }
        load
    }
}

impl FromStr for Dish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<char> = Grid::from_str(s)?;
        let mut dish = Dish {
            rounds_in_rows: vec![vec![]; grid.height()],
            rounds_in_cols: vec![vec![]; grid.width()],
            ..Default::default()
        };

        let mut col_interval_markers = vec![vec![-1]; grid.width()];

        for row in 0..grid.height() {
            let mut interval_markers = vec![-1];
            for (col, v) in grid.locations[row].iter().enumerate() {
                match v {
                    '#' => {
                        col_interval_markers[col].push(row as i8);
                        interval_markers.push(col as i8);
                    }
                    'O' => {
                        // we start always with NORTH, so we want cols first
                        // dish.rounds_in_rows[row].push(dish.rounds.len());
                        dish.rounds_in_cols[col].push(row as i8);
                    }
                    _ => {}
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

// It's surprising that this is only slightly faster.
#[derive(Debug, Default, Clone)]
pub struct BitDish {
    rounds: Vec<u128>,
    cubes: Vec<u128>,
    height: usize,
    left_border_mask: u128,
    right_border_mask: u128,
}

impl BitDish {
    fn total_load(&self) -> u32 {
        self.rounds
            .iter()
            .enumerate()
            .map(|(i, r)| (self.height - i) as u32 * r.count_ones())
            .sum()
    }

    pub fn cycle(&mut self, count: usize) -> u32 {
        let mut cache: FxHashMap<u128, usize> = FxHashMap::default();
        let mut loads: Vec<u32> = Vec::with_capacity(500);
        for cycle_idx in 0..count {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            let load = self.total_load();

            loads.push(load);

            // make a key with the last ~8~ 4 loads
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

                    let rem = (count - cycle_idx) % period;
                    // we need to advance by rem in loads from the last index - 1
                    return loads[*e + rem - 1];
                }
            }
        }

        // we should never make it here
        unreachable!("how did this happen?")
    }

    fn tilt_north(&mut self) {
        let mut rows = VecDeque::from_iter(1..self.height);

        while let Some(row) = rows.pop_front() {
            let target_row = row - 1;
            let moves_available =
                self.rounds[row] & !self.rounds[target_row] & !self.cubes[target_row];

            if moves_available != 0 {
                self.rounds[row] &= !moves_available;
                self.rounds[target_row] |= moves_available;

                if target_row > 0 {
                    rows.push_front(target_row);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut rows = Vec::from_iter(0..(self.height - 1));
        while let Some(row) = rows.pop() {
            let target_row = row + 1;
            let moves_available =
                self.rounds[row] & !self.rounds[target_row] & !self.cubes[target_row];

            if moves_available != 0 {
                self.rounds[row] &= !moves_available;
                self.rounds[target_row] |= moves_available;

                if target_row < self.height - 1 {
                    rows.push(target_row);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut rows = Vec::from_iter(0..self.height);
        while let Some(row) = rows.pop() {
            let cubes = self.cubes[row];
            let rounds = self.rounds[row];
            let moves_available = rounds & !((rounds | cubes) >> 1) & self.left_border_mask;
            if moves_available != 0 {
                self.rounds[row] = rounds & !moves_available | moves_available << 1;
                rows.push(row);
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut rows = Vec::from_iter(0..self.height);
        while let Some(row) = rows.pop() {
            let cubes = self.cubes[row];
            let rounds = self.rounds[row];
            let moves_available = rounds & !((rounds | cubes) << 1) & self.right_border_mask;
            if moves_available != 0 {
                self.rounds[row] = rounds & !moves_available | moves_available >> 1;
                rows.push(row);
            }
        }
    }
}

impl FromStr for BitDish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let height = lines.len();
        let width = lines[0].len();

        let mut rounds = vec![0; height];
        let mut cubes = vec![0; height];

        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        cubes[row] |= 1 << (width - col - 1);
                    }
                    'O' => rounds[row] |= 1 << (width - col - 1),
                    _ => {}
                }
            }
        }

        let left_border_mask = !(1 << (width - 1));
        let right_border_mask = !1;

        Ok(Self {
            rounds,
            cubes,
            height,
            left_border_mask,
            right_border_mask,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParabolicReflectorDish {
    dish: BitDish,
}

impl FromStr for ParabolicReflectorDish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dish = BitDish::from_str(s)?;

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
        dish.tilt_north();
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
