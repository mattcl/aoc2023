use std::{hash::Hash, str::FromStr, sync::Arc, thread};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::Cardinal, geometry::Location};
use rayon::prelude::*;
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
    idx: usize,
    location: Location,
    neighbors: Vec<(usize, usize)>,
    layer: usize,
    best: usize,
}

impl Node {
    pub fn new(idx: usize, location: Location) -> Self {
        Self {
            idx,
            location,
            neighbors: Vec::default(),
            layer: 0,
            best: 0,
        }
    }
}

// this is maybe too tuned to the input shape everyone was given, but a way to
// make this perhaps more general would be to use a vec instead of the array
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerSet {
    layer_counts: [u8; 15],
}

impl LayerSet {
    pub fn increment(&mut self, layer: usize) {
        self.layer_counts[layer] += 1;
    }

    pub fn decrement(&mut self, layer: usize) {
        self.layer_counts[layer] -= 1;
    }

    pub fn remaining_nodes_at_layer(&self, layer: usize) -> bool {
        self.layer_counts[layer] != 0
    }

    pub fn any_above(&self, layer: usize) -> bool {
        ((layer + 1)..self.layer_counts.len()).any(|l| self.remaining_nodes_at_layer(l))
    }
}

#[derive(Debug, Clone)]
pub struct ALongWalkGen<const N: usize> {
    p1: usize,
    p2: usize,
}

impl<const N: usize> ALongWalkGen<N> {
    pub fn make_base_graph(grid: &Grid<Tile>) -> Vec<Node> {
        let mut graph: Vec<Node> = Vec::default();

        let start = Node::new(0, Location::new(0, 1));

        let end = Node::new(1, Location::new(grid.height() - 1, grid.width() - 2));

        graph.push(start);
        graph.push(end);

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
                        graph.push(Node::new(graph.len(), loc));
                    }
                }
            }
        }

        graph
    }

    pub fn populate_graph_with_slopes(base_graph: &[Node], grid: &Grid<Tile>) -> Vec<Node> {
        let mut graph = base_graph.to_owned();
        let mut translation = FxHashMap::default();

        for n in graph.iter() {
            translation.insert(n.location, n.idx);
        }

        // for each neighbor, pathfind to its neighbors in each direction
        let results = (0..graph.len())
            .into_par_iter()
            .map(|idx| {
                let neighbors =
                    Self::explore_to_neighbors_with_slopes(idx, &graph, &translation, grid);
                (idx, neighbors)
            })
            .collect::<Vec<_>>();

        for (idx, neighbors) in results {
            graph[idx].best = neighbors
                .iter()
                .map(|(_, d)| d)
                .max()
                .copied()
                .unwrap_or_default();
            graph[idx].neighbors = neighbors;
        }

        graph
    }

    pub fn populate_graph_without_slopes(base_graph: &[Node], grid: &Grid<Tile>) -> Vec<Node> {
        let mut graph = base_graph.to_owned();
        let mut translation = FxHashMap::default();

        for n in graph.iter() {
            translation.insert(n.location, n.idx);
        }

        // for each neighbor, pathfind to its neighbors in each direction
        let results = (0..graph.len())
            .into_par_iter()
            .map(|idx| {
                let neighbors =
                    Self::explore_to_neighbors_without_slopes(idx, &graph, &translation, grid);
                (idx, neighbors)
            })
            .collect::<Vec<_>>();

        for (idx, neighbors) in results {
            graph[idx].best = neighbors
                .iter()
                .map(|(_, d)| d)
                .max()
                .copied()
                .unwrap_or_default();
            graph[idx].neighbors = neighbors;
        }

        graph
    }

    pub fn explore_to_neighbors_with_slopes(
        idx: usize,
        graph: &[Node],
        translation: &FxHashMap<Location, usize>,
        grid: &Grid<Tile>,
    ) -> Vec<(usize, usize)> {
        let mut out = Vec::default();
        let mut seen: FxHashSet<Location> = FxHashSet::default();

        let start = graph[idx].location;

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
                    if l != start && translation.contains_key(&l) {
                        let n_idx = translation.get(&l).unwrap();
                        out.push((*n_idx, dist + 1));
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
        idx: usize,
        graph: &[Node],
        translation: &FxHashMap<Location, usize>,
        grid: &Grid<Tile>,
    ) -> Vec<(usize, usize)> {
        let mut out = Vec::default();
        let mut seen: FxHashSet<Location> = FxHashSet::default();

        let start = graph[idx].location;

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
                    if l != start && translation.contains_key(&l) {
                        let n_idx = translation.get(&l).unwrap();
                        out.push((*n_idx, dist + 1));
                    } else {
                        next.push((l, dist + 1));
                    }
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        out
    }

    pub fn compute_layer_set_and_update_nodes(end: usize, graph: &mut Vec<Node>) -> LayerSet {
        let mut ls = LayerSet::default();

        let len = graph.len();

        for idx in 0..len {
            let layer = Self::dist_to_end(idx, end, graph);
            ls.increment(layer);
            graph[idx].layer = layer;
        }

        ls
    }

    pub fn dist_to_end(start: usize, end: usize, graph: &[Node]) -> usize {
        let mut cur = vec![(start, 0)];
        let mut next: Vec<(usize, usize)> = Vec::default();
        let mut seen = 0_u64;

        while !cur.is_empty() {
            for (idx, dist) in cur.drain(..) {
                if idx == end {
                    return dist;
                }

                seen |= 1_u64 << idx;

                next.extend(
                    graph[idx]
                        .neighbors
                        .iter()
                        .filter(|(n, _)| (1_u64 << n) & seen == 0)
                        .map(|(n, _)| (*n, dist + 1)),
                );
            }

            std::mem::swap(&mut cur, &mut next);
        }

        usize::MAX
    }

    pub fn longest_distance(graph: &[Node], mut layer_set: LayerSet, sloped: bool) -> usize {
        // we're going to use the layer set to eliminate situations where we are
        // forced to descend towards the end because otherwise we would not be
        // able to cross a particular layer again
        layer_set.decrement(graph[0].layer);

        // Determine the first node after the start node
        let (second, second_dist) = graph[0].neighbors[0];

        // Determine the first node before the end node, if possible.
        // In the case of the example input, and possibly the real input, it's
        // maybe not possible to walk back from the end node for part 1 because
        // of a slope. While we could attempt to find the node that leads to the
        // end, part 1 is an insignificant amount of the total runtime, so I'm
        // not going to bother.
        let (end, end_dist, initial_seen) = if graph[1].neighbors.is_empty() {
            (1, 0, 1 | 1_u64 << second)
        } else {
            let n = graph[1].neighbors[0];
            (n.0, n.1, 0b11 | 1_u64 << second)
        };

        // we're also going to use the best theoretical score to prune early, if
        // possible
        let theoretical_best = graph.iter().map(|n| n.best).sum::<usize>() - graph[1].best;

        // We're going to leverage the fact that I have enough CPU cores to run
        // the recursive searches in parallel from several starting locations,
        // so we're going to dive down a few more levels to come up with a set
        // of starting points/conditions to do in parallel.
        let mut starting_points = Vec::with_capacity(1000);
        let mut next = Vec::with_capacity(1000);
        starting_points.push((
            second,
            second_dist + end_dist,
            initial_seen,
            layer_set,
            theoretical_best,
        ));
        for _depth in 2..N {
            next.extend(
                starting_points
                    .drain(..)
                    .flat_map(|(idx, dist, seen, mut ls, best)| {
                        ls.decrement(graph[idx].layer);
                        let best_remaining = if best > 0 {
                            best - graph[idx].best
                        } else {
                            best
                        };
                        graph[idx]
                            .neighbors
                            .iter()
                            .filter(move |(fidx, _)| 1_u64 << fidx & seen == 0)
                            .map(move |(fidx, fdist)| {
                                (
                                    *fidx,
                                    dist + fdist,
                                    seen | 1_u64 << fidx,
                                    ls,
                                    best_remaining,
                                )
                            })
                    }),
            );
            std::mem::swap(&mut starting_points, &mut next);
        }

        if sloped {
            starting_points
                .par_iter()
                .map(|(start, dist, seen, ls, best_remaining)| {
                    let mut longest = 0;
                    Self::longest_recur_sloped(
                        *start,
                        *dist,
                        end,
                        graph,
                        ls,
                        *best_remaining,
                        *seen,
                        &mut longest,
                    );
                    longest
                })
                .max()
                .unwrap_or_default()
        } else {
            starting_points
                .par_iter()
                .map(|(start, dist, seen, ls, best_remaining)| {
                    let mut longest = 0;
                    Self::longest_recur(
                        *start,
                        *dist,
                        end,
                        graph,
                        ls,
                        *best_remaining,
                        *seen,
                        &mut longest,
                    );
                    longest
                })
                .max()
                .unwrap_or_default()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn longest_recur_sloped(
        start: usize,
        cur_cost: usize,
        goal: usize,
        graph: &[Node],
        layer_set: &LayerSet,
        theoretical_remaining: usize,
        seen: u64,
        longest: &mut usize,
    ) {
        if start == goal {
            *longest = (*longest).max(cur_cost);
            return;
        }

        let node = &graph[start];
        let theoretical_remaining = theoretical_remaining - node.best;

        if cur_cost + theoretical_remaining < *longest {
            return;
        }

        // we're doing this as a u64 solely to avoid the hashing or array lookup
        // overhead, which cuts the runtime from 600ms to 150ms
        let mask = 1_u64 << start;
        let next_seen = seen | mask;

        let mut next_layer_set = *layer_set;
        next_layer_set.decrement(node.layer);
        let can_move_away_from_end = next_layer_set.remaining_nodes_at_layer(node.layer);

        for (next_idx, dist) in node.neighbors.iter() {
            let next_node = &graph[*next_idx];

            if !can_move_away_from_end && next_node.layer > node.layer {
                continue;
            }

            if (1_u64 << next_idx) & next_seen == 0 {
                Self::longest_recur_sloped(
                    *next_idx,
                    cur_cost + dist,
                    goal,
                    graph,
                    &next_layer_set,
                    theoretical_remaining,
                    next_seen,
                    longest,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn longest_recur(
        start: usize,
        cur_cost: usize,
        goal: usize,
        graph: &[Node],
        layer_set: &LayerSet,
        theoretical_remaining: usize,
        seen: u64,
        longest: &mut usize,
    ) {
        if start == goal {
            *longest = (*longest).max(cur_cost);
            return;
        }

        let node = &graph[start];
        let theoretical_remaining = theoretical_remaining - node.best;

        if cur_cost + theoretical_remaining < *longest {
            return;
        }

        // we're doing this as a u64 solely to avoid the hashing or array lookup
        // overhead, which cuts the runtime from 600ms to 150ms
        let mask = 1_u64 << start;
        let next_seen = seen | mask;

        let mut next_layer_set = *layer_set;
        next_layer_set.decrement(node.layer);
        let can_move_away_from_end = next_layer_set.remaining_nodes_at_layer(node.layer);

        // Edit: this is apparently an invalid assumption on all inputs.
        // we know we have to visit all the nodes, so bail if we have any
        // unvisited nodes above us and we would have to move towards the end
        // if !can_move_away_from_end && next_layer_set.any_above(node.layer) {
        //     return;
        // }

        for (next_idx, dist) in node.neighbors.iter() {
            let next_node = &graph[*next_idx];

            if !can_move_away_from_end && next_node.layer > node.layer {
                continue;
            }

            if (1_u64 << next_idx) & next_seen == 0 {
                Self::longest_recur(
                    *next_idx,
                    cur_cost + dist,
                    goal,
                    graph,
                    &next_layer_set,
                    theoretical_remaining,
                    next_seen,
                    longest,
                );
            }
        }
    }
}

impl<const N: usize> FromStr for ALongWalkGen<N> {
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

        // Threading this ended up being unnecessary, since the p2 time
        // significantly dwarfs the p1 time, but I'm too lazy to remove this
        // now.
        let p1_handle = thread::spawn(move || {
            let mut g = Self::populate_graph_with_slopes(&graph, &grid);
            let layer_set = Self::compute_layer_set_and_update_nodes(1, &mut g);
            Self::longest_distance(&g, layer_set, true)
        });

        let p2_handle = thread::spawn(move || {
            let mut g = Self::populate_graph_without_slopes(&p2_graph, &p2_grid);
            let layer_set = Self::compute_layer_set_and_update_nodes(1, &mut g);
            Self::longest_distance(&g, layer_set, false)
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

impl<const N: usize> Problem for ALongWalkGen<N> {
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

pub type ALongWalk = ALongWalkGen<10>;

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
        let solution = ALongWalkGen::<5>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(94, 154));
    }
}
