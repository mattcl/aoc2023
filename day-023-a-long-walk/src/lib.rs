use std::{hash::Hash, str::FromStr, sync::Arc, thread};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::Cardinal, geometry::Location};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    SlopeNorth,
    SlopeSouth,
    SlopeEast,
    SlopeWest,
    Empty,
    Wall,
}

impl Tile {
    pub fn permitted(&self, dir: &Cardinal) -> bool {
        match self {
            Self::SlopeNorth => *dir == Cardinal::North,
            Self::SlopeSouth => *dir == Cardinal::South,
            Self::SlopeWest => *dir == Cardinal::West,
            Self::SlopeEast => *dir == Cardinal::East,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    location: Location,
    tile: Tile,
    neighbors: Vec<(Location, usize)>,
}

impl Node {
    pub fn new(location: Location, tile: Tile) -> Self {
        Self {
            location,
            tile,
            neighbors: Vec::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ALongWalk {
    p1: usize,
    p2: usize,
}

impl ALongWalk {
    pub fn make_base_graph(grid: &Grid<Tile>) -> FxHashMap<Location, Node> {
        let mut graph: FxHashMap<Location, Node> = FxHashMap::default();

        let start = Node::new(Location::new(0, 1), Tile::Empty);

        let end = Node::new(
            Location::new(grid.height() - 1, grid.width() - 2),
            Tile::Empty,
        );

        graph.insert(Location::new(0, 1), start);
        graph.insert(Location::new(grid.height() - 1, grid.width() - 2), end);

        // add all the nodes to the graph that have more than two neighbors as
        // these are now junction points
        for r in 1..(grid.height() - 1) {
            for c in 1..(grid.width() - 1) {
                let tile = grid.locations[r][c];
                if tile != Tile::Wall {
                    // if we have more than two neighbors
                    let loc = Location::new(r, c);
                    if grid
                        .cardinal_neighbors(&loc)
                        .filter(|(_, _, t)| **t != Tile::Wall)
                        .count()
                        > 2
                    {
                        graph.insert(loc, Node::new(loc, tile));
                    }
                }
            }
        }

        graph
    }

    pub fn populate_graph_with_slopes(
        base_graph: &FxHashMap<Location, Node>,
        grid: &Grid<Tile>,
    ) -> FxHashMap<Location, Node> {
        let mut graph = base_graph.clone();

        // for each neighbor, pathfind to its neighbors in each direction
        let junction_locations = graph.keys().copied().collect::<Vec<_>>();
        for loc in junction_locations {
            let neighbors = Self::explore_to_neighbors_with_slopes(loc, &graph, grid);
            let n = graph.get_mut(&loc).unwrap();
            n.neighbors = neighbors;
        }

        graph
    }

    pub fn populate_graph_without_slopes(
        base_graph: &FxHashMap<Location, Node>,
        grid: &Grid<Tile>,
    ) -> FxHashMap<Location, Node> {
        let mut graph = base_graph.clone();

        // for each neighbor, pathfind to its neighbors in each direction
        let junction_locations = graph.keys().copied().collect::<Vec<_>>();
        for loc in junction_locations {
            let neighbors = Self::explore_to_neighbors_without_slopes(loc, &graph, grid);
            let n = graph.get_mut(&loc).unwrap();
            n.neighbors = neighbors;
        }

        graph
    }

    pub fn explore_to_neighbors_with_slopes(
        start: Location,
        graph: &FxHashMap<Location, Node>,
        grid: &Grid<Tile>,
    ) -> Vec<(Location, usize)> {
        let mut out = Vec::default();
        let mut seen: FxHashSet<Location> = FxHashSet::default();

        let mut cur = vec![(start, 0)];
        let mut next: Vec<(Location, usize)> = Vec::default();

        // bfs
        while !cur.is_empty() {
            for (loc, dist) in cur.drain(..) {
                if seen.contains(&loc) {
                    continue;
                }
                seen.insert(loc);

                let tile = grid.get(&loc).unwrap();

                for (_, l, _) in grid
                    .cardinal_neighbors(&loc)
                    .filter(|(d, _, t)| **t != Tile::Wall && tile.permitted(d))
                {
                    if l != start && graph.contains_key(&l) {
                        out.push((l, dist + 1));
                    } else {
                        next.push((l, dist + 1));
                    }
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        out
    }

    pub fn explore_to_neighbors_without_slopes(
        start: Location,
        graph: &FxHashMap<Location, Node>,
        grid: &Grid<Tile>,
    ) -> Vec<(Location, usize)> {
        let mut out = Vec::default();
        let mut seen: FxHashSet<Location> = FxHashSet::default();

        let mut cur = vec![(start, 0)];
        let mut next: Vec<(Location, usize)> = Vec::default();

        // bfs
        while !cur.is_empty() {
            for (loc, dist) in cur.drain(..) {
                if seen.contains(&loc) {
                    continue;
                }
                seen.insert(loc);

                for (_, l, _) in grid
                    .cardinal_neighbors(&loc)
                    .filter(|(_, _, t)| **t != Tile::Wall)
                {
                    if l != start && graph.contains_key(&l) {
                        out.push((l, dist + 1));
                    } else {
                        next.push((l, dist + 1));
                    }
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        out
    }

    pub fn longest_distance(
        start: Location,
        end: Location,
        graph: &FxHashMap<Location, Node>,
    ) -> usize {
        let mut seen = FxHashSet::default();
        let mut longest = 0;

        Self::longest_recur(&start, 0, &end, graph, &mut seen, &mut longest);

        longest
    }

    pub fn longest_recur(
        start: &Location,
        cur_cost: usize,
        goal: &Location,
        graph: &FxHashMap<Location, Node>,
        seen: &mut FxHashSet<Location>,
        longest: &mut usize,
    ) {
        if start == goal {
            *longest = (*longest).max(cur_cost);
            return;
        }

        if seen.contains(start) {
            return;
        }

        seen.insert(*start);

        if let Some(node) = graph.get(start) {
            for (loc, dist) in node.neighbors.iter() {
                Self::longest_recur(loc, cur_cost + dist, goal, graph, seen, longest);
            }
        }

        seen.remove(start);
    }
}

impl FromStr for ALongWalk {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations = s
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        '>' => Tile::SlopeEast,
                        '<' => Tile::SlopeWest,
                        '^' => Tile::SlopeNorth,
                        'v' => Tile::SlopeSouth,
                        _ => unreachable!("Unexpected character"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let grid = Arc::new(Grid::new(locations));

        let graph = Arc::new(Self::make_base_graph(&grid));

        let p2_grid = grid.clone();
        let p2_graph = graph.clone();

        let start = Location::new(0, 1);
        let end = Location::new(grid.height() - 1, grid.width() - 2);

        let p1_handle = thread::spawn(move || {
            let g = Self::populate_graph_with_slopes(&graph, &grid);
            Self::longest_distance(start, end, &g)
        });

        let p2_handle = thread::spawn(move || {
            let g = Self::populate_graph_without_slopes(&p2_graph, &p2_grid);
            Self::longest_distance(start, end, &g)
        });

        let p1 = p1_handle
            .join()
            .map_err(|e| anyhow!("failed to solve p1: {:?}", e))?;
        let p2 = p2_handle
            .join()
            .map_err(|e| anyhow!("failed to solve p1: {:?}", e))?;

        Ok(Self { p1, p2 })
    }
}

impl Problem for ALongWalk {
    const DAY: usize = 23;
    const TITLE: &'static str = "a long walk";
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
        let solution = ALongWalk::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2438, 6658));
    }

    #[test]
    fn example() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let solution = ALongWalk::solve(input).unwrap();
        assert_eq!(solution, Solution::new(94, 154));
    }
}
