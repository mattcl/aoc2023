use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::directions::Relative;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, none_of},
    combinator,
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use num_prime::nt_funcs::factorize64;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    left: u64,
    right: u64,
    ends_with_a: bool,
    ends_with_z: bool,
}

#[allow(clippy::type_complexity)]
fn parse_node(input: &str) -> IResult<&str, ((u64, bool, bool), (u64, u64))> {
    separated_pair(
        combinator::map(alphanumeric1, |s: &str| {
            (xxh3_64(s.as_bytes()), s.ends_with('A'), s.ends_with('Z'))
        }),
        tag(" = "),
        delimited(
            tag("("),
            combinator::map(
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                |(left, right): (&str, &str)| (xxh3_64(left.as_bytes()), xxh3_64(right.as_bytes())),
            ),
            tag(")"),
        ),
    )(input)
}

fn parse_nodes(input: &str) -> IResult<&str, FxHashMap<u64, Node>> {
    fold_many1(
        preceded(newline, parse_node),
        FxHashMap::default,
        |mut m, ((key, ends_with_a, ends_with_z), (left, right))| {
            let node = Node {
                left,
                right,
                ends_with_a,
                ends_with_z,
            };
            m.insert(key, node);
            m
        },
    )(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Relative>> {
    many1(combinator::map_res(none_of("\n"), |ch| {
        Relative::try_from(ch)
    }))(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Relative>, FxHashMap<u64, Node>)> {
    separated_pair(parse_instructions, newline, parse_nodes)(input)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    key: u64,
    node: Node,
}

#[derive(Debug, Clone)]
pub struct HauntedWasteland {
    instructions: Vec<Relative>,
    mapping: FxHashMap<u64, Node>,
}

impl HauntedWasteland {
    pub fn steps_from(&self, start: u64, end: u64) -> usize {
        let mut count = 0;

        let insts = self.instructions.iter().cycle();
        let mut cur = start;
        let mut cur_node = self.mapping.get(&start).unwrap();

        #[allow(clippy::explicit_counter_loop)]
        for inst in insts {
            if cur == end {
                break;
            }

            match inst {
                Relative::Left => cur = cur_node.left,
                Relative::Right => cur = cur_node.right,
                _ => unreachable!(),
            }

            cur_node = self.mapping.get(&cur).unwrap();
            count += 1;
        }

        count
    }

    pub fn ghost_steps_from(&self) -> u64 {
        let mut factors: FxHashSet<u64> = FxHashSet::default();

        let factor_groups: Vec<_> = self
            .mapping
            .par_iter()
            .filter(|(_, v)| v.ends_with_a)
            .map(|(k, _)| {
                let count = self.get_first_instance(*k);
                factorize64(count as u64)
            })
            .collect();

        for g in factor_groups {
            factors.extend(g.keys().copied());
        }

        let mut count = 1;
        for factor in factors {
            count *= factor;
        }

        count
    }

    /// This is making a massive assumption that we never hit Z multiple times
    /// in a given _cycle_, because if that were true, it would feel like it'd
    /// be possible to perhaps get a smaller number or friendlier number to
    /// factorize in the later steps
    pub fn get_first_instance(&self, start: u64) -> u32 {
        let mut count = 0;
        let insts = self.instructions.iter().cycle();
        let mut cur = start;
        let mut cur_node = self.mapping.get(&cur).unwrap();

        #[allow(clippy::explicit_counter_loop)]
        for inst in insts {
            if cur_node.ends_with_z {
                return count;
            }

            match inst {
                Relative::Left => cur = cur_node.left,
                Relative::Right => cur = cur_node.right,
                _ => unreachable!(),
            }

            cur_node = self.mapping.get(&cur).unwrap();
            count += 1;
        }

        unreachable!();
    }
}

impl FromStr for HauntedWasteland {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (instructions, mapping)) = parse(s).map_err(|e| e.to_owned())?;
        Ok(Self {
            instructions,
            mapping,
        })
    }
}

impl Problem for HauntedWasteland {
    const DAY: usize = 8;
    const TITLE: &'static str = "haunted wasteland";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let start = xxh3_64(b"AAA");
        let end = xxh3_64(b"ZZZ");
        Ok(self.steps_from(start, end))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.ghost_steps_from())
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
        let solution = HauntedWasteland::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(12361, 18215611419223));
    }

    #[test]
    fn part_one_example() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let mut inst = HauntedWasteland::instance(input).unwrap();
        let ans = inst.part_one().unwrap();
        assert_eq!(ans, 2);
    }

    #[test]
    fn part_two_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let mut inst = HauntedWasteland::instance(input).unwrap();
        let ans = inst.part_two().unwrap();
        assert_eq!(ans, 6);
    }
}
