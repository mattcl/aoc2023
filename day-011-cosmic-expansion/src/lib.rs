use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::{AocPoint, Point2D};
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Galaxy {
    original: Point2D<i64>,
    one: Point2D<i64>,
    one_million: Point2D<i64>,
}

impl Galaxy {
    pub fn new(location: Point2D<i64>) -> Self {
        Self {
            original: location,
            one: location,
            one_million: location,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CosmicExpansion {
    single_expansion: i64,
    million_expansion: i64,
}

impl FromStr for CosmicExpansion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::default();

        let lines = s.trim().lines().collect::<Vec<_>>();
        let width = lines[0].len() as i64;

        let mut num_empty_rows = 0_i64;

        let mut empty_cols_raw = FxHashSet::from_iter(0..width);

        for (row, line) in lines.into_iter().enumerate() {
            let mut seen_row = false;
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    seen_row = true;
                    let mut galaxy = Galaxy::new(Point2D::new(col as i64, row as i64));
                    galaxy.one.y += num_empty_rows;
                    galaxy.one_million.y += num_empty_rows * 999999;
                    galaxies.push(galaxy);
                    empty_cols_raw.remove(&(col as i64));
                }
            }

            if !seen_row {
                num_empty_rows += 1;
            }
        }

        let mut empty_cols = empty_cols_raw.into_iter().sorted().collect::<Vec<_>>();
        empty_cols.push(width);

        let mut counts = vec![0_i64; width as usize];

        for ((_, col1), (idx2, col2)) in empty_cols.iter().enumerate().tuple_windows() {
            for j in *col1..*col2 {
                counts[j as usize] = idx2 as i64;
            }
        }

        for g in galaxies.iter_mut() {
            let multiple = counts[g.original.x as usize];
            g.one.x += multiple;
            g.one_million.x += 999_999 * multiple;
        }

        // we're 13 microseconds up to this point, and then the ~85k
        // combinations add another 100 or so microseconds.
        let Point2D {
            x: single_expansion,
            y: million_expansion,
        } = galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| {
                Point2D::new(
                    a.one.manhattan_dist(&b.one),
                    a.one_million.manhattan_dist(&b.one_million),
                )
            })
            .sum::<Point2D<i64>>();

        Ok(Self {
            single_expansion,
            million_expansion,
        })
    }
}

impl Problem for CosmicExpansion {
    const DAY: usize = 11;
    const TITLE: &'static str = "cosmic expansion";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.single_expansion)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.million_expansion)
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
        let solution = CosmicExpansion::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(9724940, 569052586852));
    }

    #[test]
    fn example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let solution = CosmicExpansion::solve(input).unwrap();
        assert_eq!(solution, Solution::new(374, 82000210));
    }
}
