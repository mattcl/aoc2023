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
    groups: Vec<u8>,
}

impl Spring {
    pub fn new(key: &str, groups: Vec<u8>) -> Self {
        Self {
            key: key.to_string(),
            groups,
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

pub fn fast_arrangements(input: &[u8], groups: &[u8]) -> usize {
    let mut damaged_segments = vec![0; input.len()];
    let mut cur_segment = 0;
    let mut first_damaged = input.len();

    for (i, ch) in input.iter().copied().enumerate() {
        if ch != b'.' {
            cur_segment += 1;
        } else {
            cur_segment = 0;
        }

        if ch == b'#' && i < first_damaged {
            first_damaged = i;
        }
        damaged_segments[i] = cur_segment;
    }

    let mut cur = vec![0; input.len() + 1];
    let mut prev = vec![0; input.len() + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=first_damaged {
        prev[i] = 1;
    }

    for (idx, count) in groups.iter().copied().enumerate() {
        let count = count as usize;
        cur[0] = 0;
        for end in 0..input.len() {
            let last = input[end];
            let mut cur_count = 0;
            if last != b'#' {
                cur_count += cur[end];
            }

            if last != b'.' {
                let next_damaged = input.get(end + 1).filter(|v| **v == b'#').is_some();
                if !next_damaged && damaged_segments[end] >= count {
                    let previous_undamaged = end
                        .checked_sub(count)
                        .map(|i| input[i])
                        .filter(|v| *v == b'#')
                        .is_none();
                    if previous_undamaged {
                        cur_count += end.checked_sub(count).map(|i| prev[i]).unwrap_or_else(|| {
                            if idx == 0 {
                                1
                            } else {
                                0
                            }
                        });
                    }
                }
            }

            cur[end + 1] = cur_count;
        }

        std::mem::swap(&mut cur, &mut prev);
    }

    prev[input.len()]
}

pub fn arrangements(
    input: &[u8],
    groups: &[u8],
    seen: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if groups.is_empty() {
        if input.contains(&b'#') {
            return 0;
        } else {
            return 1;
        }
    }

    if input.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        return 0;
    }

    let key = (input.len(), groups.len());

    if let Some(cached) = seen.get(&key) {
        return *cached;
    }

    // This is weird, but we already know we have this pattern because of the
    // check at the top to bail out of the method before checking the cache.
    let num_arrangements = if let [first, remain @ ..] = input {
        match first {
            b'?' => match groups {
                [] => {
                    if remain.iter().all(|ch| *ch != b'#') {
                        1
                    } else {
                        0
                    }
                }
                [v, remaining_groups @ ..] => {
                    let needed = *v as usize - 1;
                    if needed > remain.len() {
                        0
                    } else if remain.iter().take(needed).all(|ch| *ch != b'.') {
                        if remain.len() == needed {
                            if remaining_groups.is_empty() {
                                1
                            } else {
                                0
                            }
                        } else if remain.len() > needed && remain[needed] != b'#' {
                            // can we fill the group?
                            // how to do this all at once?
                            arrangements(&remain[(needed + 1)..], remaining_groups, seen)
                                + arrangements(remain, groups, seen)
                        } else {
                            arrangements(remain, groups, seen)
                        }
                    } else {
                        arrangements(remain, groups, seen)
                    }
                }
            },
            b'#' => match groups {
                [] => 0,
                [v, remaining_groups @ ..] => {
                    let needed = *v as usize - 1;
                    if needed > remain.len() {
                        0
                    } else if remain.iter().take(needed).all(|ch| *ch != b'.') {
                        if remain.len() == needed {
                            if remaining_groups.is_empty() {
                                1
                            } else {
                                0
                            }
                        } else if remain.len() > needed && remain[needed] != b'#' {
                            arrangements(&remain[(needed + 1)..], remaining_groups, seen)
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
            },
            b'.' => {
                if let Some((idx, _)) = remain.iter().enumerate().find(|(_, ch)| **ch != b'.') {
                    arrangements(&remain[idx..], groups, seen)
                } else if groups.is_empty() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    } else {
        unreachable!()
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
            .map(|s| fast_arrangements(s.key.as_bytes(), &s.groups))
            .sum()
    }

    pub fn count_long_arrangements(&self) -> usize {
        self.springs
            .par_iter()
            .map(|s| {
                let long_key = [&s.key].iter().cycle().take(5).join("?");
                let long_groups: Vec<_> = s
                    .groups
                    .iter()
                    .copied()
                    .cycle()
                    .take(5 * s.groups.len())
                    .collect();

                fast_arrangements(long_key.as_bytes(), &long_groups)
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
