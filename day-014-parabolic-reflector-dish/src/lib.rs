use std::str::FromStr;

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
    fn total_load_p1(&self) -> u32 {
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
            for col in 0..grid.width() {
                match grid.locations[row][col] {
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

#[derive(Debug, Clone)]
pub struct ParabolicReflectorDish {
    dish: Dish,
}

impl FromStr for ParabolicReflectorDish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dish = Dish::from_str(s)?;

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
        Ok(dish.total_load_p1())
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
