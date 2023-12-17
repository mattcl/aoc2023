use std::{hash::Hash, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid, directions::Cardinal, geometry::Location, pathing::dijkstra::dijkstra,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    location: Location,
    num_straight: usize,
    facing: Cardinal,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            location: Location::default(),
            num_straight: 1,
            facing: Cardinal::East,
        }
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct ClumsyCrucible {
    grid: Grid<u32>,
}

impl ClumsyCrucible {
    pub fn minimize(&self) -> u32 {
        let start = Node::default();
        let end = Location::new(self.grid.height() - 1, self.grid.width() - 1);
        let result = dijkstra(
            &start,
            &mut |node| {
                let old = *node;
                self.grid.cardinal_neighbors(&node.location).filter_map(
                    move |(facing, location, cost)| {
                        if facing == old.facing {
                            if old.num_straight > 2 {
                                None
                            } else {
                                Some((
                                    Node {
                                        location,
                                        facing,
                                        num_straight: old.num_straight + 1,
                                    },
                                    *cost,
                                ))
                            }
                        } else if facing != old.facing.opposite() {
                            Some((
                                Node {
                                    location,
                                    facing,
                                    num_straight: 1,
                                },
                                *cost,
                            ))
                        } else {
                            None
                        }
                    },
                )
            },
            &mut |node| node.location == end,
        );

        result.cost().unwrap_or_default()
    }

    pub fn ultra_minimize(&self) -> u32 {
        let start = Node::default();
        let end = Location::new(self.grid.height() - 1, self.grid.width() - 1);
        let result = dijkstra(
            &start,
            &mut |node| {
                let old = *node;
                self.grid.cardinal_neighbors(&node.location).filter_map(
                    move |(facing, location, cost)| {
                        if facing == old.facing {
                            if old.num_straight > 9 {
                                None
                            } else {
                                Some((
                                    Node {
                                        location,
                                        facing,
                                        num_straight: old.num_straight + 1,
                                    },
                                    *cost,
                                ))
                            }
                        } else if facing != old.facing.opposite() {
                            // attempt to move three additional spots from this
                            // neighbor in the determined direction
                            let mut total_cost = *cost;
                            if let Some((four_away, v)) = self
                                .grid
                                .cardinal_neighbor(&location, facing)
                                .and_then(|(next_loc, v)| {
                                    total_cost += *v;
                                    self.grid.cardinal_neighbor(&next_loc, facing)
                                })
                                .and_then(|(next_loc, v)| {
                                    total_cost += *v;
                                    self.grid.cardinal_neighbor(&next_loc, facing)
                                })
                            {
                                total_cost += v;
                                Some((
                                    Node {
                                        location: four_away,
                                        facing,
                                        num_straight: 4,
                                    },
                                    total_cost,
                                ))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    },
                )
            },
            &mut |node| node.location == end,
        );

        result.cost().unwrap_or_default()
    }
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

        let grid = Grid::new(values);
        Ok(Self { grid })
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
        Ok(self.minimize())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.ultra_minimize())
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
