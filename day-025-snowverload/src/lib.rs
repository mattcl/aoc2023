use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{collections::FxIndexSet, conversions::strs::str_to_u64};
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    combinator,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use pathfinding::prelude::edmonds_karp_dense;
use rand::{seq::SliceRandom, thread_rng};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Default, Clone)]
pub struct RawNode {
    name: u64,
    neighbors: Vec<u64>,
}

fn parse_raw_node(input: &str) -> IResult<&str, RawNode> {
    combinator::map(
        separated_pair(
            combinator::map(alpha1, str_to_u64),
            tag(": "),
            separated_list1(complete::char(' '), combinator::map(alpha1, str_to_u64)),
        ),
        |(name, neighbors)| RawNode { name, neighbors },
    )(input)
}

fn parse_raw_nodes(input: &str) -> IResult<&str, Vec<RawNode>> {
    separated_list1(newline, parse_raw_node)(input)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Node {
    index: usize,
    neighbors: FxHashSet<usize>,
}

#[derive(Debug, Clone)]
pub struct Snowverload {
    verticies: Vec<u64>,
    edges: Vec<((u64, u64), i64)>,
    neighbors: FxHashMap<u64, FxHashSet<u64>>,
}

impl Snowverload {
    pub fn cleave(&mut self) -> usize {
        let mut rng = thread_rng();

        loop {
            let start = self.verticies.choose(&mut rng).unwrap();
            let end = self.verticies.choose(&mut rng).unwrap();

            if start == end {
                continue;
            }

            let (_, capacity, min_cut) =
                edmonds_karp_dense(&self.verticies, start, end, self.edges.iter().copied());
            if capacity == 3 && min_cut.len() == 3 {
                for ((l, r), _) in min_cut.iter() {
                    {
                        let left = self.neighbors.get_mut(l).unwrap();
                        left.remove(r);
                    }
                    {
                        let right = self.neighbors.get_mut(r).unwrap();
                        right.remove(l);
                    }
                }

                // pick one of the cuts to bfs from
                let ((l, r), _) = min_cut[0];
                return self.bfs(l) * self.bfs(r);
            }
        }
    }

    fn bfs(&self, start: u64) -> usize {
        let mut seen: FxHashSet<u64> = FxHashSet::default();
        let mut cur: Vec<u64> = Vec::default();
        let mut next: Vec<u64> = Vec::default();
        cur.push(start);

        while !cur.is_empty() {
            for c in cur.drain(..) {
                if seen.contains(&c) {
                    continue;
                }
                seen.insert(c);

                if let Some(neighbors) = self.neighbors.get(&c) {
                    next.extend(neighbors.iter().copied());
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        seen.len()
    }
}

impl FromStr for Snowverload {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, raw_nodes) = parse_raw_nodes(s).map_err(|e| e.to_owned())?;

        let mut neighbors: FxHashMap<u64, FxHashSet<u64>> = FxHashMap::default();
        let mut verticies_raw = raw_nodes
            .iter()
            .map(|n| n.name)
            .collect::<FxIndexSet<u64>>();
        for n in raw_nodes.iter() {
            for ne in n.neighbors.iter() {
                verticies_raw.insert(*ne);
            }
        }

        for node in raw_nodes.iter() {
            let e = neighbors.entry(node.name).or_default();
            for n in node.neighbors.iter() {
                e.insert(*n);
            }

            for n in node.neighbors.iter() {
                let e = neighbors.entry(*n).or_default();
                e.insert(node.name);
            }
        }

        let edges = raw_nodes
            .iter()
            .flat_map(|node| node.neighbors.iter().map(|n| ((node.name, *n), 1)))
            .chain(
                raw_nodes
                    .iter()
                    .flat_map(|node| node.neighbors.iter().map(|n| ((*n, node.name), 1))),
            )
            .collect::<Vec<_>>();
        let verticies = verticies_raw.into_iter().collect::<Vec<_>>();

        Ok(Self {
            verticies,
            edges,
            neighbors,
        })
    }
}

impl Problem for Snowverload {
    const DAY: usize = 25;
    const TITLE: &'static str = "snowverload";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = &'static str;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.cleave())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok("no part 2 for day 25")
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
        let solution = Snowverload::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(582590, "no part 2 for day 25"));
    }

    #[test]
    fn example() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let solution = Snowverload::solve(input).unwrap();
        assert_eq!(solution, Solution::new(54, "no part 2 for day 25"));
    }
}
