use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::collections::Grid;

#[derive(Debug, Clone)]
pub struct Mirror {
    grid: Grid<char>,
}

impl FromStr for Mirror {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

impl Mirror {
    pub fn reflect_horizontal(&self) -> Option<usize> {
        'outer: for left_col in 0..(self.grid.width() - 1) {
            let right_col = left_col + 1;
            for row in 0..self.grid.height() {
                let limit = (left_col + 1).min(self.grid.width() - right_col);
                for offset in 0..limit {
                    if self.grid.locations[row][left_col - offset]
                        != self.grid.locations[row][right_col + offset]
                    {
                        continue 'outer;
                    }
                }
            }

            return Some(left_col + 1);
        }

        None
    }

    pub fn reflect_vertical(&self) -> Option<usize> {
        'outer: for top_row in 0..(self.grid.height() - 1) {
            let bot_row = top_row + 1;
            for col in 0..self.grid.width() {
                let limit = (top_row + 1).min(self.grid.height() - bot_row);
                for offset in 0..limit {
                    if self.grid.locations[top_row - offset][col]
                        != self.grid.locations[bot_row + offset][col]
                    {
                        continue 'outer;
                    }
                }
            }

            return Some((top_row + 1) * 100);
        }

        None
    }

    pub fn reflect_horizontal_one_off(&self) -> Option<usize> {
        'outer: for left_col in 0..(self.grid.width() - 1) {
            let mut count_off = 0;
            let right_col = left_col + 1;
            for row in 0..self.grid.height() {
                let limit = (left_col + 1).min(self.grid.width() - right_col);
                for offset in 0..limit {
                    if self.grid.locations[row][left_col - offset]
                        != self.grid.locations[row][right_col + offset]
                    {
                        count_off += 1;
                        if count_off > 1 {
                            continue 'outer;
                        }
                    }
                }
            }

            if count_off == 1 {
                return Some(left_col + 1);
            }
        }

        None
    }

    pub fn reflect_vertical_one_off(&self) -> Option<usize> {
        'outer: for top_row in 0..(self.grid.height() - 1) {
            let mut count_off = 0;
            let bot_row = top_row + 1;
            for col in 0..self.grid.width() {
                let limit = (top_row + 1).min(self.grid.height() - bot_row);
                for offset in 0..limit {
                    if self.grid.locations[top_row - offset][col]
                        != self.grid.locations[bot_row + offset][col]
                    {
                        count_off += 1;
                        if count_off > 1 {
                            continue 'outer;
                        }
                    }
                }
            }

            if count_off == 1 {
                return Some((top_row + 1) * 100);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct PointOfIncidence {
    mirrors: Vec<Mirror>,
}

impl PointOfIncidence {
    pub fn summarize(&self) -> usize {
        self.mirrors
            .iter()
            .map(|m| m.reflect_vertical().unwrap_or(0) + m.reflect_horizontal().unwrap_or(0))
            .sum()
    }

    pub fn fix_smudge(&self) -> usize {
        self.mirrors
            .iter()
            .map(|m| {
                m.reflect_vertical_one_off().unwrap_or(0)
                    + m.reflect_horizontal_one_off().unwrap_or(0)
            })
            .sum()
    }
}

impl FromStr for PointOfIncidence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s
            .trim()
            .split("\n\n")
            .map(Mirror::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { mirrors })
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
        Ok(self.summarize())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.fix_smudge())
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
