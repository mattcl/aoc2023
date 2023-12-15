use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::collections::FxIndexMap;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, alpha1},
    combinator,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Remove,
    Assign(u8),
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        combinator::map(complete::char('-'), |_| Op::Remove),
        combinator::map(preceded(complete::char('='), complete::u8), |v| {
            Op::Assign(v)
        }),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction<'a> {
    bucket: usize,
    label: &'a str,
    op: Op,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    combinator::map(
        tuple((
            combinator::map(alpha1, |s: &str| {
                (
                    s.as_bytes()
                        .iter()
                        .fold(0, |acc, ch| ((acc + *ch as usize) * 17) % 256),
                    s,
                )
            }),
            parse_op,
        )),
        |((bucket, label), op)| Instruction { bucket, label, op },
    )(input)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry<'a> {
    label: &'a str,
    focal: u8,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Bucket<'a> {
    values: FxIndexMap<&'a str, Entry<'a>>,
}

impl<'a> Bucket<'a> {
    pub fn insert(&mut self, entry: Entry<'a>) {
        self.values.insert(entry.label, entry);
    }

    pub fn remove(&mut self, label: &str) {
        self.values.shift_remove(label);
    }

    #[inline]
    pub fn focal_sum(&self) -> usize {
        self.values
            .iter()
            .enumerate()
            .map(|(v_idx, (_, v))| (v_idx + 1) * v.focal as usize)
            .sum::<usize>()
    }
}

/// In testing this is a few percent faster than the bucket using the index map
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ListBucket<'a> {
    values: VecDeque<Entry<'a>>,
}

impl<'a> ListBucket<'a> {
    pub fn insert(&mut self, entry: Entry<'a>) {
        if let Some((idx, _)) = self.values.iter().find_position(|v| v.label == entry.label) {
            self.values[idx] = entry;
        } else {
            self.values.push_back(entry);
        }
    }

    pub fn remove(&mut self, label: &str) {
        if let Some((idx, _)) = self.values.iter().find_position(|v| v.label == label) {
            self.values.remove(idx);
        }
    }

    #[inline]
    pub fn focal_sum(&self) -> usize {
        self.values
            .iter()
            .enumerate()
            .map(|(v_idx, v)| (v_idx + 1) * v.focal as usize)
            .sum::<usize>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HASHMap<'a> {
    buckets: Vec<ListBucket<'a>>,
}

impl<'a> Default for HASHMap<'a> {
    fn default() -> Self {
        Self {
            buckets: vec![ListBucket::default(); 256],
        }
    }
}

impl<'a> HASHMap<'a> {
    pub fn insert(&mut self, bucket: usize, entry: Entry<'a>) {
        self.buckets[bucket].insert(entry);
    }

    pub fn remove(&mut self, bucket: usize, label: &str) {
        self.buckets[bucket].remove(label);
    }

    pub fn focusing_power(&self) -> usize {
        self.buckets
            // .par_iter()
            .iter()
            .enumerate()
            .map(|(bucket_idx, bucket)| (bucket_idx + 1) * bucket.focal_sum())
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct LensLibrary {
    p1: u32,
    p2: usize,
}

impl FromStr for LensLibrary {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hm = HASHMap::default();
        let mut p1 = 0;
        for step in s.trim().split(',') {
            p1 += step
                .as_bytes()
                .iter()
                .fold(0, |acc, ch| ((acc + *ch as u32) * 17) % 256);
            let (_, inst) = parse_instruction(step).map_err(|e| e.to_owned())?;
            match inst.op {
                Op::Remove => hm.remove(inst.bucket, inst.label),
                Op::Assign(v) => hm.insert(
                    inst.bucket,
                    Entry {
                        label: inst.label,
                        focal: v,
                    },
                ),
            }
        }
        Ok(Self {
            p1,
            p2: hm.focusing_power(),
        })
    }
}

impl Problem for LensLibrary {
    const DAY: usize = 15;
    const TITLE: &'static str = "lens library";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
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
        let solution = LensLibrary::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(517015, 286104));
    }

    #[test]
    fn example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let solution = LensLibrary::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1320, 145));
    }
}
