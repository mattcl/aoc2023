use std::{hash::Hash, str::FromStr, sync::Arc, thread};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid, directions::Cardinal, geometry::Location, pathing::dijkstra::dijkstra,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    location: Location,
    facing: Cardinal,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            location: Location::default(),
            facing: Cardinal::East,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blocks {
    grid: Grid<u32>,
}

impl Blocks {
    pub fn all_in_direction(&self, start: Location, dir: Cardinal, min: usize, max: usize) -> impl Iterator<Item=(Node, u32)> + '_ {
        // precompute cost below the min
        let mut cost = (1..min)
            .map(|i| {
                match dir {
                    Cardinal::North if start.row >= i => {
                        let new_row = start.row - i;
                        self.grid.locations[new_row][start.col]
                    },
                    Cardinal::South if self.grid.height() > start.row + i => {
                        let new_row = start.row + i;
                        self.grid.locations[new_row][start.col]
                    },
                    Cardinal::West if start.col >= i => {
                        let new_col = start.col - i;
                        self.grid.locations[start.row][new_col]
                    },
                    Cardinal::East if self.grid.width() > start.col + i => {
                        let new_col = start.col + i;
                        self.grid.locations[start.row][new_col]
                    },
                    _ => 0
                }
            })
            .sum();

        (min..=max)
            .filter_map(move |i| {
                match dir {
                    Cardinal::North if start.row >= i => {
                        let new_row = start.row - i;
                        cost += self.grid.locations[new_row][start.col];
                        Some((
                            Node { location: Location::new(new_row, start.col), facing: dir },
                            cost
                        ))
                    },
                    Cardinal::South if self.grid.height() > start.row + i => {
                        let new_row = start.row + i;
                        cost += self.grid.locations[new_row][start.col];
                        Some((
                            Node { location: Location::new(new_row, start.col), facing: dir },
                            cost
                        ))
                    },
                    Cardinal::West if start.col >= i => {
                        let new_col = start.col - i;
                        cost += self.grid.locations[start.row][new_col];
                        Some((
                            Node { location: Location::new(start.row, new_col), facing: dir },
                            cost
                        ))
                    },
                    Cardinal::East if self.grid.width() > start.col + i => {
                        let new_col = start.col + i;
                        cost += self.grid.locations[start.row][new_col];
                        Some((
                            Node { location: Location::new(start.row, new_col), facing: dir },
                            cost
                        ))
                    },
                    _ => None
                }
            })
    }

    pub fn minimize(&self, min: usize, max: usize) -> u32 {
        let start = Node::default();
        let end = Location::new(self.grid.height() - 1, self.grid.width() - 1);
        let mut first = true;
        let result = dijkstra(
            &start,
            &mut |node| {
                let old = *node;

                if first {
                    first = false;
                    [Cardinal::East, Cardinal::South]
                } else {
                    [old.facing.left(), old.facing.right()]
                }
                    .into_iter()
                    .map(move |dir| self.all_in_direction(old.location, dir, min, max))
                    .flatten()
            },
            &mut |node| node.location == end,
        );

        result.cost().unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct ClumsyCrucible {
    p1: u32,
    p2: u32,
}

impl FromStr for ClumsyCrucible {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| ch.to_digit(10).unwrap_or_default())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Ordinarilly I wouldn't do this, but given how long each part takes
        // and that each part is actually independent, this at least mostly
        // makes the time to solve the same as the time to solve part 2
        let blocks = Arc::new(Blocks {
            grid: Grid::new(values),
        });
        let p2_blocks = blocks.clone();

        let p1_handle = thread::spawn(move || blocks.minimize(1, 3));
        let p2_handle = thread::spawn(move || p2_blocks.minimize(4, 10));

        let p1 = p1_handle
            .join()
            .map_err(|e| anyhow!("failed to solve p1: {:?}", e))?;
        let p2 = p2_handle
            .join()
            .map_err(|e| anyhow!("failed to solve p2: {:?}", e))?;

        Ok(Self { p1, p2 })
    }
}

impl Problem for ClumsyCrucible {
    const DAY: usize = 17;
    const TITLE: &'static str = "clumsy crucible";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
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
        let solution = ClumsyCrucible::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1004, 1171));
    }

    #[test]
    fn example() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let solution = ClumsyCrucible::solve(input).unwrap();
        assert_eq!(solution, Solution::new(102, 94));
    }

    #[test]
    fn example2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let solution = ClumsyCrucible::solve(input).unwrap();
        assert_eq!(solution, Solution::new(59, 71));
    }
}
