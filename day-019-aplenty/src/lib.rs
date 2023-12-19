use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::Interval;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, one_of},
    combinator,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use rustc_hash::FxHashMap;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    X,
    M,
    A,
    S,
}

fn parse_key(input: &str) -> IResult<&str, Key> {
    combinator::map(one_of("xmas"), |ch| match ch {
        'x' => Key::X,
        'm' => Key::M,
        'a' => Key::A,
        's' => Key::S,
        _ => unreachable!(),
    })(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decision {
    Reject,
    Accept,
    Workflow(u64),
}

fn parse_decision(input: &str) -> IResult<&str, Decision> {
    alt((
        combinator::map(complete::char('R'), |_| Decision::Reject),
        combinator::map(complete::char('A'), |_| Decision::Accept),
        combinator::map(alpha1, |v: &str| Decision::Workflow(xxh3_64(v.as_bytes()))),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule {
    AlwaysAccept,
    AlwaysReject,
    LessThan {
        key: Key,
        value: i64,
        decision: Decision,
    },
    GreaterThan {
        key: Key,
        value: i64,
        decision: Decision,
    },
    Workflow {
        name: u64,
    },
}

impl Rule {
    pub fn process(&self, part: &Part) -> Option<Decision> {
        match self {
            Self::LessThan {
                key,
                value,
                decision,
            } if part.get(key) < *value => Some(*decision),
            Self::GreaterThan {
                key,
                value,
                decision,
            } if part.get(key) > *value => Some(*decision),
            Self::Workflow { name } => Some(Decision::Workflow(*name)),
            Self::AlwaysAccept => Some(Decision::Accept),
            Self::AlwaysReject => Some(Decision::Reject),
            _ => None,
        }
    }
}

fn parse_less_than(input: &str) -> IResult<&str, Rule> {
    combinator::map(
        separated_pair(
            separated_pair(parse_key, complete::char('<'), complete::i64),
            complete::char(':'),
            parse_decision,
        ),
        |((key, value), decision)| Rule::LessThan {
            key,
            value,
            decision,
        },
    )(input)
}

fn parse_greater_than(input: &str) -> IResult<&str, Rule> {
    combinator::map(
        separated_pair(
            separated_pair(parse_key, complete::char('>'), complete::i64),
            complete::char(':'),
            parse_decision,
        ),
        |((key, value), decision)| Rule::GreaterThan {
            key,
            value,
            decision,
        },
    )(input)
}

fn parse_always_accept(input: &str) -> IResult<&str, Rule> {
    combinator::map(complete::char('A'), |_| Rule::AlwaysAccept)(input)
}

fn parse_always_reject(input: &str) -> IResult<&str, Rule> {
    combinator::map(complete::char('R'), |_| Rule::AlwaysReject)(input)
}

fn parse_rule_workflow(input: &str) -> IResult<&str, Rule> {
    combinator::map(alpha1, |v: &str| Rule::Workflow {
        name: xxh3_64(v.as_bytes()),
    })(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((
        parse_always_accept,
        parse_always_reject,
        parse_less_than,
        parse_greater_than,
        parse_rule_workflow,
    ))(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(complete::char(','), parse_rule)(input)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn process(&self, part: &Part) -> Decision {
        for rule in self.rules.iter() {
            if let Some(decision) = rule.process(part) {
                return decision;
            }
        }
        unreachable!("All workflows should make a decision");
    }
}

fn parse_workflow(input: &str) -> IResult<&str, (u64, Workflow)> {
    combinator::map(
        tuple((
            alpha1,
            delimited(complete::char('{'), parse_rules, complete::char('}')),
        )),
        |(k, rules)| (xxh3_64(k.as_bytes()), Workflow { rules }),
    )(input)
}

fn parse_workflows(input: &str) -> IResult<&str, FxHashMap<u64, Workflow>> {
    fold_many1(
        terminated(parse_workflow, newline),
        FxHashMap::default,
        |mut m, (k, v)| {
            m.insert(k, v);
            m
        },
    )(input)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    pub fn get(&self, key: &Key) -> i64 {
        match key {
            Key::X => self.x,
            Key::M => self.m,
            Key::A => self.a,
            Key::S => self.s,
        }
    }

    pub fn total_rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_x(input: &str) -> IResult<&str, i64> {
    delimited(tag("x="), complete::i64, complete::char(','))(input)
}

fn parse_m(input: &str) -> IResult<&str, i64> {
    delimited(tag("m="), complete::i64, complete::char(','))(input)
}

fn parse_a(input: &str) -> IResult<&str, i64> {
    delimited(tag("a="), complete::i64, complete::char(','))(input)
}

fn parse_s(input: &str) -> IResult<&str, i64> {
    preceded(tag("s="), complete::i64)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    combinator::map(
        delimited(
            complete::char('{'),
            tuple((parse_x, parse_m, parse_a, parse_s)),
            complete::char('}'),
        ),
        |(x, m, a, s)| Part { x, m, a, s },
    )(input)
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(newline, parse_part)(input)
}

fn parse_input(input: &str) -> IResult<&str, (FxHashMap<u64, Workflow>, Vec<Part>)> {
    separated_pair(parse_workflows, newline, parse_parts)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntervalSet {
    x: Interval<i64>,
    m: Interval<i64>,
    a: Interval<i64>,
    s: Interval<i64>,
}

impl Default for IntervalSet {
    fn default() -> Self {
        Self {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Aplenty {
    workflows: FxHashMap<u64, Workflow>,
    parts: Vec<Part>,
}

impl Aplenty {
    pub fn sum_accepted(&self) -> i64 {
        let in_workflow = xxh3_64(b"in");
        let mut total = 0;
        for part in self.parts.iter() {
            let mut workflow = in_workflow;
            loop {
                let decision = self.workflows.get(&workflow).unwrap().process(part);
                match decision {
                    Decision::Accept => {
                        total += part.total_rating();
                        break;
                    }
                    Decision::Reject => {
                        break;
                    }
                    Decision::Workflow(next_workflow) => {
                        workflow = next_workflow;
                    }
                }
            }
        }
        total
    }

    pub fn combo_accepted(&self) -> i64 {
        let in_workflow = xxh3_64(b"in");
        let mut intervals: Vec<(IntervalSet, Decision, usize)> =
            vec![(IntervalSet::default(), Decision::Workflow(in_workflow), 0)];

        let mut accepted: Vec<IntervalSet> = Vec::default();

        while let Some((interval_set, old_decision, rule_idx)) = intervals.pop() {
            match old_decision {
                Decision::Accept => {
                    accepted.push(interval_set);
                }
                Decision::Workflow(next_workflow) => {
                    let workflow = self.workflows.get(&next_workflow).unwrap();
                    if rule_idx >= workflow.rules.len() {
                        continue;
                    }

                    let rule = &workflow.rules[rule_idx];

                    match rule {
                        Rule::AlwaysAccept => {
                            accepted.push(interval_set);
                        }
                        Rule::Workflow { name } => {
                            intervals.push((interval_set, Decision::Workflow(*name), 0));
                        }
                        Rule::LessThan {
                            key,
                            value,
                            decision,
                        } => match key {
                            Key::X => {
                                if interval_set.x.contains_value(*value - 1) {
                                    let mut next_set = interval_set;
                                    next_set.x = Interval::new(next_set.x.start, *value - 1);
                                    intervals.push((next_set, *decision, 0));
                                }

                                if interval_set.x.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.x = Interval::new(*value, next_set.x.end);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }
                            }
                            Key::M => {
                                if interval_set.m.contains_value(*value - 1) {
                                    let mut next_set = interval_set;
                                    next_set.m = Interval::new(next_set.m.start, *value - 1);
                                    intervals.push((next_set, *decision, 0));
                                }

                                if interval_set.m.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.m = Interval::new(*value, next_set.m.end);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }
                            }
                            Key::A => {
                                if interval_set.a.contains_value(*value - 1) {
                                    let mut next_set = interval_set;
                                    next_set.a = Interval::new(next_set.a.start, *value - 1);
                                    intervals.push((next_set, *decision, 0));
                                }

                                if interval_set.a.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.a = Interval::new(*value, next_set.a.end);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }
                            }
                            Key::S => {
                                if interval_set.s.contains_value(*value - 1) {
                                    let mut next_set = interval_set;
                                    next_set.s = Interval::new(next_set.s.start, *value - 1);
                                    intervals.push((next_set, *decision, 0));
                                }

                                if interval_set.s.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.s = Interval::new(*value, next_set.s.end);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }
                            }
                        },
                        Rule::GreaterThan {
                            key,
                            value,
                            decision,
                        } => match key {
                            Key::X => {
                                if interval_set.x.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.x = Interval::new(next_set.x.start, *value);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }

                                if interval_set.x.contains_value(*value + 1) {
                                    let mut next_set = interval_set;
                                    next_set.x = Interval::new(*value + 1, next_set.x.end);
                                    intervals.push((next_set, *decision, 0));
                                }
                            }
                            Key::M => {
                                if interval_set.m.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.m = Interval::new(next_set.m.start, *value);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }

                                if interval_set.m.contains_value(*value + 1) {
                                    let mut next_set = interval_set;
                                    next_set.m = Interval::new(*value + 1, next_set.m.end);
                                    intervals.push((next_set, *decision, 0));
                                }
                            }
                            Key::A => {
                                if interval_set.a.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.a = Interval::new(next_set.a.start, *value);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }

                                if interval_set.a.contains_value(*value + 1) {
                                    let mut next_set = interval_set;
                                    next_set.a = Interval::new(*value + 1, next_set.a.end);
                                    intervals.push((next_set, *decision, 0));
                                }
                            }
                            Key::S => {
                                if interval_set.s.contains_value(*value) {
                                    let mut next_set = interval_set;
                                    next_set.s = Interval::new(next_set.s.start, *value);
                                    intervals.push((next_set, old_decision, rule_idx + 1));
                                }

                                if interval_set.s.contains_value(*value + 1) {
                                    let mut next_set = interval_set;
                                    next_set.s = Interval::new(*value + 1, next_set.s.end);
                                    intervals.push((next_set, *decision, 0));
                                }
                            }
                        },
                        Rule::AlwaysReject => { /* do nothing */ }
                    }
                }
                _ => {}
            }
        }

        accepted
            .iter()
            .map(|iset| iset.x.width() * iset.m.width() * iset.a.width() * iset.s.width())
            .sum()
    }
}

impl FromStr for Aplenty {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (workflows, parts)) = parse_input(s).map_err(|e| e.to_owned())?;
        Ok(Self { workflows, parts })
    }
}

impl Problem for Aplenty {
    const DAY: usize = 19;
    const TITLE: &'static str = "aplenty";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.sum_accepted())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.combo_accepted())
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
        let solution = Aplenty::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(434147, 136146366355609));
    }

    #[test]
    fn example() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let solution = Aplenty::solve(input).unwrap();
        assert_eq!(solution, Solution::new(19114, 167409079868000));
    }

    #[test]
    fn parsing() {
        let (_, _r) = parse_rule("a<2006:qkq").unwrap();
        let (_, _w) = parse_workflow("px{a<2006:qkq,m>2090:A,rfg}").unwrap();
    }
}
