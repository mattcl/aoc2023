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
        let mut seen_odd = FxHashSet::default();
        let mut seen_even = FxHashSet::default();
        let mut cur = Vec::default();
        let mut next = Vec::default();

        cur.push(self.start);

        for step in 0..steps {
            for loc in cur.drain(..) {
                if step % 2 == 0 {
                    if seen_even.contains(&loc) {
                        continue;
                    }
                    seen_even.insert(loc);
                } else {
                    if seen_odd.contains(&loc) {
                        continue;
                    }
                    seen_odd.insert(loc);
                }
                next.extend(self.grid.cardinal_neighbors(&loc).filter_map(|(_, l, t)| {
                    if *t == Tile::Garden
                        && ((step % 2 == 0 && !seen_odd.contains(&l))
                            || (step % 2 == 1 && !seen_even.contains(&l)))
                    {
                        Some(l)
                    } else {
                        None
                    }
                }));
            }
            std::mem::swap(&mut cur, &mut next);
        }

        // it's actually faster to just remove the duplicates here than it is
        // for cur and next to be hashsets
        let unique = FxHashSet::from_iter(cur);
        unique.len() + seen_even.len()
    }

    pub fn infinite_bfs(&self, steps: usize) -> i64 {
        let mut seen_odd = FxHashSet::default();
        let mut seen_even = FxHashSet::default();
        let mut cur = Vec::default();
        let mut next = Vec::default();

        // directions will have different meanings, but the same effects, even
        // if tranversal order is different
        let start: Point2D<i64> = Point2D::new(self.start.row as i64, self.start.col as i64);

        let mut counts = vec![];

        cur.push(start);

        let rem = steps % self.grid.width();

        for step in 0..steps {
            if step % self.grid.width() == rem {
                let prev = if step % 2 == 0 {
                    seen_even.len()
                } else {
                    seen_odd.len()
                };
                // it's actually faster to just remove the duplicates whenever
                // we're inserting than it is for cur and next to be hashsets
                let unique = FxHashSet::from_iter(cur.iter());
                counts.push(unique.len() + prev);

                if counts.len() == 3 {
                    break;
                }
            }

            for loc in cur.drain(..) {
                if step % 2 == 0 {
                    if seen_even.contains(&loc) {
                        continue;
                    }
                    seen_even.insert(loc);
                } else {
                    if seen_odd.contains(&loc) {
                        continue;
                    }
                    seen_odd.insert(loc);
                }

                next.extend(loc.cardinal_neighbors().filter_map(|(_, l)| {
                    let row = l.x.rem_euclid(self.grid.height() as i64) as usize;
                    let col = l.y.rem_euclid(self.grid.width() as i64) as usize;
                    if self.grid.locations[row][col] == Tile::Garden
                        && ((step % 2 == 0 && !seen_odd.contains(&l))
                            || (step % 2 == 1 && !seen_even.contains(&l)))
                    {
                        Some(l)
                    } else {
                        None
                    }
                }));
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
