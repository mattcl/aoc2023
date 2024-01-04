use std::{hash::Hash, str::FromStr, sync::Arc, thread};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, geometry::Location, pathing::dijkstra::bucket_dijkstra};

// we actually only need to know which directions our left and right are, since
// we're going to precompute all the nodes between min-max so n/s e/w are
// indistinguishable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    orientation: Orientation,
    location: Location,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            location: Location::default(),
            orientation: Orientation::Horizontal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blocks {
    grid: Grid<u8>,
}

impl Blocks {
    pub fn horizontal(
        &self,
        start: Location,
        min: usize,
        max: usize,
    ) -> impl Iterator<Item = (Node, usize)> + '_ {
        let mut cost_west = 0;
        let mut cost_east = 0;
        for i in 1..min.min(start.col + 1) {
            let new_col = start.col - i;
            cost_west += self.grid.locations[start.row][new_col];
        }

        for i in (start.col + 1)..(start.col + min).min(self.grid.width()) {
            cost_east += self.grid.locations[start.row][i];
        }

        let max_west = max.min(start.col);
        let max_east = max.min(self.grid.width() - 1 - start.col);

        (min..=max_east)
            .map(move |i| {
                let new_col = start.col + i;
                cost_east += self.grid.locations[start.row][new_col];
                (
                    Node {
                        location: Location::new(start.row, new_col),
                        orientation: Orientation::Horizontal,
                    },
                    cost_east as usize,
                )
            })
            .chain((min..=max_west).map(move |i| {
                let new_col = start.col - i;
                cost_west += self.grid.locations[start.row][new_col];
                (
                    Node {
                        location: Location::new(start.row, new_col),
                        orientation: Orientation::Horizontal,
                    },
                    cost_west as usize,
                )
            }))
    }

    pub fn vertical(
        &self,
        start: Location,
        min: usize,
        max: usize,
    ) -> impl Iterator<Item = (Node, usize)> + '_ {
        let mut cost_north = 0;
        let mut cost_south = 0;
        for i in 1..min.min(start.row + 1) {
            let new_row = start.row - i;
            cost_north += self.grid.locations[new_row][start.col];
        }

        for i in (start.row + 1)..(start.row + min).min(self.grid.height()) {
            cost_south += self.grid.locations[i][start.col];
        }

        let max_north = max.min(start.row);
        let max_south = max.min(self.grid.height() - 1 - start.row);

        (min..=max_south)
            .map(move |i| {
                let new_row = start.row + i;
                cost_south += self.grid.locations[new_row][start.col];
                (
                    Node {
                        location: Location::new(new_row, start.col),
                        orientation: Orientation::Vertical,
                    },
                    cost_south as usize,
                )
            })
            .chain((min..=max_north).map(move |i| {
                let new_row = start.row - i;
                cost_north += self.grid.locations[new_row][start.col];
                (
                    Node {
                        location: Location::new(new_row, start.col),
                        orientation: Orientation::Vertical,
                    },
                    cost_north as usize,
                )
            }))
    }

    pub fn minimize(&self, min: usize, max: usize) -> usize {
        let start = Node::default();
        let end = Location::new(self.grid.height() - 1, self.grid.width() - 1);
        let mut first = true;
        let result = bucket_dijkstra(
            &start,
            &mut |node| {
                let location = node.location;

                // we can avoid the vec allocations with Box<dyn Iterator...>,
                // but that doesn't actually give us performance gains.
                if first {
                    first = false;
                    self.horizontal(location, min, max)
                        .chain(self.vertical(location, min, max))
                        .collect::<Vec<_>>()
                } else if node.orientation == Orientation::Horizontal {
                    self.vertical(location, min, max).collect::<Vec<_>>()
                } else {
                    self.horizontal(location, min, max).collect::<Vec<_>>()
                }
            },
            &mut |node| node.location == end,
        );

        result.cost().unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct ClumsyCrucible {
    p1: usize,
    p2: usize,
}

impl FromStr for ClumsyCrucible {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| ch.to_digit(10).unwrap_or_default() as u8)
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
    type P1 = usize;
    type P2 = usize;

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
