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

        let mut empty_rows: Vec<i64> = Vec::default();
        let mut seen_col: FxHashSet<i64> = FxHashSet::default();

        let mut largest_col = 0;

        for (row, line) in s.trim().lines().enumerate() {
            let mut seen_row = false;
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    seen_row = true;
                    seen_col.insert(col as i64);
                    galaxies.push(Galaxy::new(Point2D::new(col as i64, row as i64)));
                    if col as i64 > largest_col {
                        largest_col = col as i64;
                    }
                }
            }

            if !seen_row {
                empty_rows.push(row as i64);
            }
        }

        for col in (0..largest_col).rev() {
            if !seen_col.contains(&col) {
                // we need to shift all galaxies beyond this row down
                for g in galaxies.iter_mut() {
                    if g.original.x > col {
                        g.one.x += 1;
                        g.one_million.x += 999_999;
                    }
                }
            }
        }

        for g in galaxies.iter_mut() {
            for (row_idx, row) in empty_rows.iter().enumerate().rev() {
                if g.original.y > *row {
                    let multiple = (row_idx + 1) as i64;
                    g.one.y += multiple;
                    g.one_million.y += 999_999 * multiple;
                    break;
                }
            }
        }

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
