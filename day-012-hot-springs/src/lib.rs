use std::str::FromStr;

use aoc_plumbing::Problem;
use itertools::Itertools;
use nom::{
    bytes::complete::take_until1,
    character::complete::{self, newline},
    combinator,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Spring {
    key: String,
    long_key: String,
    groups: Vec<u8>,
    long_groups: Vec<u8>,
}

impl Spring {
    pub fn new(key: &str, groups: Vec<u8>) -> Self {
        let long_key = [key].iter().cycle().take(5).join("?");
        let long_groups = groups
            .iter()
            .copied()
            .cycle()
            .take(5 * groups.len())
            .collect();

        Self {
            key: key.to_string(),
            long_key,
            groups,
            long_groups,
        }
    }
}

fn parse_groups(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(complete::char(','), complete::u8)(input)
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    combinator::map(
        separated_pair(take_until1(" "), complete::char(' '), parse_groups),
        |(key, groups)| Spring::new(key, groups),
    )(input)
}

fn parse_springs(input: &str) -> IResult<&str, Vec<Spring>> {
    separated_list1(newline, parse_spring)(input)
}

pub fn arrangements(
    input: &[u8],
    groups: &[u8],
    group_num_left: Option<u8>,
    seen: &mut FxHashMap<(usize, usize, Option<u8>), usize>,
) -> usize {
    let key = (input.len(), groups.len(), group_num_left);
    if let Some(cached) = seen.get(&key) {
        return *cached;
    }

    let num_arrangements = match input {
        [] => match group_num_left {
            Some(0) | None if groups.is_empty() => 1,
            _ => 0,
        },
        [first, remain @ ..] => match first {
            b'?' => match group_num_left {
                Some(0) => arrangements(remain, groups, None, seen),
                Some(x) => arrangements(remain, groups, Some(x - 1), seen),
                None => match groups {
                    [] => arrangements(remain, groups, None, seen),
                    [v, remaining_groups @ ..] => {
                        arrangements(remain, remaining_groups, Some(v - 1), seen)
                            + arrangements(remain, groups, None, seen)
                    }
                },
            },
            b'#' => match group_num_left {
                Some(0) => 0,
                Some(x) => arrangements(remain, groups, Some(x - 1), seen),
                None => match groups {
                    [] => 0,
                    [v, remaining_groups @ ..] => {
                        arrangements(remain, remaining_groups, Some(v - 1), seen)
                    }
                },
            },
            b'.' => match group_num_left {
                Some(0) | None => arrangements(remain, groups, None, seen),
                _ => 0,
            },
            _ => unreachable!(),
        },
    };

    seen.insert(key, num_arrangements);

    num_arrangements
}

#[derive(Debug, Clone)]
pub struct HotSprings {
    springs: Vec<Spring>,
}

impl HotSprings {
    pub fn count_arrangements(&self) -> usize {
        self.springs
            .par_iter()
            .map(|s| {
                let mut seen = FxHashMap::default();
                arrangements(s.key.as_bytes(), &s.groups, None, &mut seen)
            })
            .sum()
    }

    pub fn count_long_arrangements(&self) -> usize {
        self.springs
            .par_iter()
            .map(|s| {
                let mut seen = FxHashMap::default();
                arrangements(s.long_key.as_bytes(), &s.long_groups, None, &mut seen)
            })
            .sum()
    }
}

impl FromStr for HotSprings {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, springs) = parse_springs(s).map_err(|e| e.to_owned())?;
        Ok(Self { springs })
    }
}

impl Problem for HotSprings {
    const DAY: usize = 12;
    const TITLE: &'static str = "hot springs";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.count_arrangements())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.count_long_arrangements())
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
        let solution = HotSprings::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(7350, 200097286528151));
    }

    #[test]
    fn example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let solution = HotSprings::solve(input).unwrap();
        assert_eq!(solution, Solution::new(21, 525152));
    }
}
