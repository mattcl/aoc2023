use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid,
    geometry::{Location, Point2D},
};

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
        let origin_map = self.distance_map(&[self.start]);
        let border_map = self.distance_map(&[
            (0, 0).into(),
            (0, self.start.col).into(),
            (0, self.grid.width() - 1).into(),
            (self.start.row, 0).into(),
            (self.start.row, self.grid.width() - 1).into(),
            (self.grid.height() - 1, 0).into(),
            (self.grid.height() - 1, self.start.col).into(),
            (self.grid.height() - 1, self.grid.width() - 1).into(),
        ]);

        // we're going to basically assume this is square from this point on
        let half = (self.grid.height() / 2) as u16;

        let mut full_odd = 0_u64;
        let mut full_even = 0_u64;
        let mut corner_odd = 0_u64;
        let mut corner_even = 0_u64;

        for r in 0..self.grid.height() {
            for c in 0..self.grid.width() {
                let origin_dist = origin_map[r][c];
                if origin_dist == u16::MAX {
                    continue;
                }

                if origin_dist % 2 == 0 {
                    full_even += 1;

                    if origin_dist > half && border_map[r][c] <= half {
                        corner_even += 1;
                    }
                } else {
                    full_odd += 1;

                    if origin_dist > half && border_map[r][c] <= half {
                        corner_odd += 1;
                    }
                }
            }
        }

        // number of grids until outer edge
        let n = (steps as u64 - half as u64) / self.grid.height() as u64;
        (n + 1).pow(2) * full_odd + n.pow(2) * full_even - (n + 1) * corner_odd + n * corner_even
    }

    pub fn distance_map(&self, start: &[Location]) -> Vec<Vec<u16>> {
        let mut steps = vec![vec![u16::MAX; self.grid.width()]; self.grid.height()];

        let mut cur = VecDeque::with_capacity(1000);
        for loc in start.iter() {
            cur.push_back((*loc, 0));
            steps[loc.row][loc.col] = 0;
        }

        while let Some((loc, dist)) = cur.pop_front() {
            for (_, l, _) in self
                .grid
                .cardinal_neighbors(&loc)
                .filter(|(_, _, t)| **t == Tile::Garden)
            {
                if steps[l.row][l.col] == u16::MAX {
                    steps[l.row][l.col] = dist + 1;
                    cur.push_back((l, dist + 1));
                }
            }
        }

        steps
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
