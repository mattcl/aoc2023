use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid,
    geometry::{Location, Point2D},
};
use rustc_hash::FxHashSet;

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
        // let mut seen = FxHashSet::default();
        let mut cur = FxHashSet::default();
        let mut next = FxHashSet::default();

        cur.insert(self.start);

        for _ in 0..steps {
            for loc in cur.drain() {
                for (_, next_loc, _) in self
                    .grid
                    .cardinal_neighbors(&loc)
                    .filter(|(_, _, t)| **t == Tile::Garden)
                {
                    next.insert(next_loc);
                }
            }
            std::mem::swap(&mut cur, &mut next);
        }

        cur.len()
    }

    pub fn infinite_bfs(&self, steps: usize) -> i64 {
        // let mut seen = FxHashSet::default();
        let mut cur = FxHashSet::default();
        let mut next = FxHashSet::default();

        // directions will have different meanings, but the same effects, even
        // if tranversal order is different
        let start: Point2D<i64> = Point2D::new(self.start.row as i64, self.start.col as i64);

        let mut counts = vec![];

        cur.insert(start);

        let rem = steps % self.grid.width();

        for step in 0..steps {
            if step % self.grid.width() == rem {
                counts.push(cur.len());

                if counts.len() == 3 {
                    break;
                }
            }

            for loc in cur.drain() {
                next.extend(
                    loc.cardinal_neighbors()
                        .filter(|(_, l)| {
                            let row = l.x.rem_euclid(self.grid.height() as i64) as usize;
                            let col = l.y.rem_euclid(self.grid.width() as i64) as usize;
                            self.grid.locations[row][col] == Tile::Garden
                        })
                        .map(|(_, l)| l),
                );
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
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.bfs(64))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.infinite_bfs(26501365))
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
