use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid,
    geometry::{Location, Point2D},
};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, Clone)]
pub struct StepCounter {
    grid: Grid<Tile>,
    start: Location,
}

impl StepCounter {
    pub fn bfs(&self, steps: usize) -> usize {
        let parity = steps % 2 == 0;
        let mut seen = vec![vec![false; self.grid.width()]; self.grid.height()];
        seen[self.start.row][self.start.col] = true;
        let mut cur = Vec::default();
        let mut next = Vec::default();

        let mut count = if parity { 1 } else { 0 };

        cur.push(self.start);

        for step in 1..=steps {
            if cur.is_empty() {
                break;
            }
            for loc in cur.drain(..) {
                next.extend(self.grid.cardinal_neighbors(&loc).filter_map(|(_, l, t)| {
                    if *t == Tile::Garden && !seen[l.row][l.col] {
                        seen[l.row][l.col] = true;
                        if parity == (step % 2 == 0) {
                            count += 1;
                        }
                        Some(l)
                    } else {
                        None
                    }
                }));
            }
            std::mem::swap(&mut cur, &mut next);
        }

        count
    }

    pub fn geometric_infinite(&self, steps: usize) -> u64 {
        // we're going to basically assume this is square from this point on
        let n = self.grid.height() as u16;
        let r = n / 2;
        let x = steps as u64 / n as u64;

        let starting_conditions = [
            // core
            (
                (self.start.row, self.start.col).into(),
                u16::MAX,
                u16::MAX,
                x * x,
                (x - 1) * (x - 1),
            ),
            // cardinal
            ((0, self.start.col).into(), n - 1, n - 1, 1, 0),
            (
                (self.grid.height() - 1, self.start.col).into(),
                n - 1,
                n - 1,
                1,
                0,
            ),
            ((self.start.row, 0).into(), n - 1, n - 1, 1, 0),
            (
                (self.start.row, self.grid.width() - 1).into(),
                n - 1,
                n - 1,
                1,
                0,
            ),
            // corners
            ((0, 0).into(), r - 1, n + r - 1, x, x - 1),
            (
                (0, self.grid.width() - 1).into(),
                r - 1,
                n + r - 1,
                x,
                x - 1,
            ),
            (
                (self.grid.height() - 1, 0).into(),
                r - 1,
                n + r - 1,
                x,
                x - 1,
            ),
            (
                (self.grid.height() - 1, self.grid.width() - 1).into(),
                r - 1,
                n + r - 1,
                x,
                x - 1,
            ),
        ];

        starting_conditions
            .into_par_iter()
            .map(
                |(start, even_limit, odd_limit, even_multiplier, odd_multiplier)| {
                    self.constrained_reachable(
                        &start,
                        even_limit,
                        odd_limit,
                        even_multiplier,
                        odd_multiplier,
                    )
                },
            )
            .sum()
    }

    pub fn constrained_reachable(
        &self,
        start: &Location,
        even_limit: u16,
        odd_limit: u16,
        even_multiplier: u64,
        odd_multiplier: u64,
    ) -> u64 {
        let mut seen = vec![vec![false; self.grid.width()]; self.grid.height()];
        let mut cur = VecDeque::with_capacity(1000);
        let mut even = 1;
        let mut odd = 0;
        cur.push_back((*start, 0));
        seen[start.row][start.col] = true;

        while let Some((loc, dist)) = cur.pop_front() {
            for (_, l, _) in self
                .grid
                .cardinal_neighbors(&loc)
                .filter(|(_, _, t)| **t == Tile::Garden)
            {
                if !seen[l.row][l.col] {
                    seen[l.row][l.col] = true;
                    let next_dist = dist + 1;

                    if next_dist % 2 == 0 {
                        if next_dist <= even_limit {
                            even += 1;
                        }
                    } else if next_dist <= odd_limit {
                        odd += 1;
                    }

                    if next_dist <= odd_limit {
                        cur.push_back((l, next_dist));
                    }
                }
            }
        }

        even as u64 * even_multiplier + odd as u64 * odd_multiplier
    }

    // my original implementation. A friend's impl convinced me the geomeric
    // solution is faster, but keeping this around as an alternative.
    pub fn infinite_bfs(&self, steps: usize) -> i64 {
        let parity = steps % 2 == 0;
        let mut cur = Vec::default();
        let mut next = Vec::default();

        let rem = steps % self.grid.width();
        let max_width = self.grid.width() * 5;
        let offset = (self.grid.width() * 2) as i32;
        // this is faster than hashing everything, apparently
        let mut seen = vec![vec![false; max_width]; max_width];

        // directions will have different meanings, but the same effects, even
        // if tranversal order is different
        let start: Point2D<i32> = Point2D::new(self.start.row as i32, self.start.col as i32);

        let mut odd_count = if parity { 1 } else { 0 };

        let mut even_count = if parity { 0 } else { 1 };

        let mut counts = vec![];

        seen[self.start.row + offset as usize][self.start.col + offset as usize] = true;
        cur.push(start);

        for step in 1..=steps {
            for loc in cur.drain(..) {
                next.extend(loc.cardinal_neighbors().filter_map(|(_, l)| {
                    let row = l.x.rem_euclid(self.grid.height() as i32) as usize;
                    let col = l.y.rem_euclid(self.grid.width() as i32) as usize;
                    let adjusted_row = (offset + l.x) as usize;
                    let adjusted_col = (offset + l.y) as usize;
                    if self.grid.locations[row][col] == Tile::Garden
                        && !seen[adjusted_row][adjusted_col]
                    {
                        seen[adjusted_row][adjusted_col] = true;
                        if parity == (step % 2 == 0) {
                            odd_count += 1;
                        } else {
                            even_count += 1;
                        }
                        Some(l)
                    } else {
                        None
                    }
                }));
            }

            if step % self.grid.width() == rem {
                if step % 2 == 1 {
                    counts.push(odd_count);
                } else {
                    counts.push(even_count);
                }

                if counts.len() == 3 {
                    break;
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        // we need three points to interpolate the polynomial
        let a = counts[0] as i64;
        let b = counts[1] as i64 - counts[0] as i64;
        let c = counts[2] as i64 - counts[1] as i64;

        let n = (steps / self.grid.width()) as i64;

        (n * (n - 1) / 2) * (c - b) + b * n + a
    }
}

impl FromStr for StepCounter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Location::default();
        let locations = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        '.' => Tile::Garden,
                        '#' => Tile::Rock,
                        'S' => {
                            start.row = row;
                            start.col = col;
                            Tile::Garden
                        }
                        _ => unreachable!("Unexpected character"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Grid::new(locations);

        Ok(Self { grid, start })
    }
}

impl Problem for StepCounter {
    const DAY: usize = 21;
    const TITLE: &'static str = "step counter";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.bfs(64))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.geometric_infinite(26501365))
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
        let solution = StepCounter::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3776, 625587097150084));
    }

    // sigh. Again, not possible to actually solve the problem for the example.
    #[test]
    fn example() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let mut inst = StepCounter::instance(input).unwrap();
        assert_eq!(inst.part_one().unwrap(), 42);
    }
}
