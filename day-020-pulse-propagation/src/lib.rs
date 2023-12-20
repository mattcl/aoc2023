use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, multispace0},
    combinator,
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};
use rustc_hash::FxHashMap;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommMod {
    FlipFlop {
        state: bool,
        destinations: Vec<u64>,
    },
    Conjunction {
        inputs: FxHashMap<u64, Pulse>,
        destinations: Vec<u64>,
    },
    Broadcast {
        destinations: Vec<u64>,
    },
}

impl CommMod {
    pub fn destinations(&self) -> &[u64] {
        match self {
            Self::FlipFlop { destinations, .. } => destinations,
            Self::Conjunction { destinations, .. } => destinations,
            Self::Broadcast { destinations } => destinations,
        }
    }
}

fn parse_destinations(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(
        tag(", "),
        combinator::map(alpha1, |v: &str| xxh3_64(v.as_bytes())),
    )(input)
}

fn parse_flip_flop(input: &str) -> IResult<&str, (u64, CommMod)> {
    combinator::map(
        separated_pair(
            preceded(
                complete::char('%'),
                combinator::map(alpha1, |v: &str| xxh3_64(v.as_bytes())),
            ),
            tag(" -> "),
            parse_destinations,
        ),
        |(name, destinations)| {
            (
                name,
                CommMod::FlipFlop {
                    state: false,
                    destinations,
                },
            )
        },
    )(input)
}

fn parse_conjunction(input: &str) -> IResult<&str, (u64, CommMod)> {
    combinator::map(
        separated_pair(
            preceded(
                complete::char('&'),
                combinator::map(alpha1, |v: &str| xxh3_64(v.as_bytes())),
            ),
            tag(" -> "),
            parse_destinations,
        ),
        |(name, destinations)| {
            (
                name,
                CommMod::Conjunction {
                    inputs: FxHashMap::default(),
                    destinations,
                },
            )
        },
    )(input)
}

fn parse_broadcaster(input: &str) -> IResult<&str, (u64, CommMod)> {
    combinator::map(
        separated_pair(
            combinator::map(tag("broadcaster"), |v: &str| xxh3_64(v.as_bytes())),
            tag(" -> "),
            parse_destinations,
        ),
        |(name, destinations)| (name, CommMod::Broadcast { destinations }),
    )(input)
}

fn parse_comm_mod(input: &str) -> IResult<&str, (u64, CommMod)> {
    alt((parse_flip_flop, parse_conjunction, parse_broadcaster))(input)
}

fn parse_comm_mods(input: &str) -> IResult<&str, FxHashMap<u64, CommMod>> {
    fold_many1(
        preceded(multispace0, parse_comm_mod),
        FxHashMap::default,
        |mut m, (key, value)| {
            m.insert(key, value);
            m
        },
    )(input)
}

#[derive(Debug, Clone)]
pub struct PulsePropagation {
    mods: FxHashMap<u64, CommMod>,
    cycle_conjunction_key: u64,
}

impl PulsePropagation {
    pub fn push_button(&self) -> usize {
        let broadcaster = xxh3_64(b"broadcaster");
        let button = xxh3_64(b"button");
        let mut mods = self.mods.clone();
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        let mut pulses = VecDeque::default();

        for _ in 0..1000 {
            pulses.push_back((button, broadcaster, Pulse::Low));
            low_pulses += 1;
            while let Some((origin, dest, pulse)) = pulses.pop_front() {
                if let Some(cur_mod) = mods.get_mut(&dest) {
                    match cur_mod {
                        CommMod::FlipFlop {
                            state,
                            destinations,
                        } => {
                            if pulse == Pulse::High {
                                continue;
                            }

                            *state = !*state;
                            let next_pulse = if *state {
                                high_pulses += destinations.len();
                                Pulse::High
                            } else {
                                low_pulses += destinations.len();
                                Pulse::Low
                            };

                            pulses.extend(
                                destinations.iter().copied().map(|d| (dest, d, next_pulse)),
                            );
                        }
                        CommMod::Conjunction {
                            inputs,
                            destinations,
                        } => {
                            inputs.insert(origin, pulse);

                            let next_pulse = if inputs.values().all(|v| *v == Pulse::High) {
                                low_pulses += destinations.len();
                                Pulse::Low
                            } else {
                                high_pulses += destinations.len();
                                Pulse::High
                            };

                            pulses.extend(
                                destinations.iter().copied().map(|d| (dest, d, next_pulse)),
                            );
                        }
                        CommMod::Broadcast { destinations } => {
                            if pulse == Pulse::High {
                                high_pulses += destinations.len();
                            } else {
                                low_pulses += destinations.len();
                            }
                            pulses.extend(destinations.iter().copied().map(|d| (dest, d, pulse)));
                        }
                    }
                }
            }
        }

        low_pulses * high_pulses
    }

    pub fn push_button_until(&self) -> usize {
        let rx = xxh3_64(b"rx");
        let broadcaster = xxh3_64(b"broadcaster");
        let button = xxh3_64(b"button");
        let mut mods = self.mods.clone();

        let rx_conjunction = self.mods.get(&self.cycle_conjunction_key).unwrap();
        let mut cycle_markers = FxHashMap::default();
        match rx_conjunction {
            CommMod::Conjunction { inputs, .. } => {
                for k in inputs.keys() {
                    cycle_markers.insert(*k, Vec::default());
                }
            }
            _ => unreachable!("We should only have attempted to get a conjunction"),
        }

        let mut count = 0;
        let mut pulses = VecDeque::default();

        loop {
            pulses.push_back((button, broadcaster, Pulse::Low));
            while let Some((origin, dest, pulse)) = pulses.pop_front() {
                if dest == rx && pulse == Pulse::Low {
                    return count;
                }
                if let Some(cur_mod) = mods.get_mut(&dest) {
                    match cur_mod {
                        CommMod::FlipFlop {
                            state,
                            destinations,
                        } => {
                            if pulse == Pulse::High {
                                continue;
                            }

                            *state = !*state;
                            let next_pulse = if *state { Pulse::High } else { Pulse::Low };

                            pulses.extend(
                                destinations.iter().copied().map(|d| (dest, d, next_pulse)),
                            );
                        }
                        CommMod::Conjunction {
                            inputs,
                            destinations,
                        } => {
                            inputs.insert(origin, pulse);

                            if dest == self.cycle_conjunction_key && pulse == Pulse::High {
                                let e = cycle_markers.entry(origin).or_default();
                                e.push(count + 1);

                                // if all the markers have at least one pulse,
                                // calculate the cycle. We're going to assume
                                // that the cycles are prime and they all start
                                // at zero, but this could be wrong
                                if cycle_markers.values().all(|v| !v.is_empty()) {
                                    return cycle_markers.values().map(|v| v[0]).product::<usize>();
                                }
                            }

                            let next_pulse = if inputs.values().all(|v| *v == Pulse::High) {
                                Pulse::Low
                            } else {
                                Pulse::High
                            };

                            pulses.extend(
                                destinations.iter().copied().map(|d| (dest, d, next_pulse)),
                            );
                        }
                        CommMod::Broadcast { destinations } => {
                            pulses.extend(destinations.iter().copied().map(|d| (dest, d, pulse)));
                        }
                    }
                }
            }
            count += 1;
        }
    }
}

impl FromStr for PulsePropagation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mut mods) = parse_comm_mods(s).map_err(|e| e.to_owned())?;

        let keys = mods.keys().copied().collect::<Vec<_>>();

        let rx = xxh3_64(b"rx");
        let mut cycle_conjunction_key = 0;

        for k in keys {
            // annoying to avoid the double mutable borrow
            let destinations = mods.get(&k).unwrap().destinations().to_vec();
            for d in destinations {
                if d == rx {
                    cycle_conjunction_key = k;
                }

                if let Some(CommMod::Conjunction { inputs, .. }) = mods.get_mut(&d) {
                    inputs.insert(k, Pulse::Low);
                }
            }
        }

        Ok(Self {
            mods,
            cycle_conjunction_key,
        })
    }
}

impl Problem for PulsePropagation {
    const DAY: usize = 20;
    const TITLE: &'static str = "pulse propagation";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.push_button())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.push_button_until())
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
        let solution = PulsePropagation::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(739960225, 231897990075517));
    }

    #[test]
    fn example() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let mut inst = PulsePropagation::instance(input).unwrap();
        assert_eq!(inst.part_one().unwrap(), 32000000);
    }
}
