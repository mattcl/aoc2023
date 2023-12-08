use std::str::FromStr;

use anyhow::Result;
use aoc_plumbing::Problem;
use aoc_std::geometry::{self, IntervalPartition};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, multispace0, multispace1, newline, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), separated_list1(space1, complete::i64))(input)
}

pub type Interval = geometry::Interval<i64>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeMapEntry {
    destination: Interval,
    source: Interval,
}

impl RangeMapEntry {
    pub fn translate(&self, input: i64) -> Option<i64> {
        if self.source.contains_value(input) {
            Some(self.destination.start + input - self.source.start)
        } else {
            None
        }
    }
}

fn parse_range_map_entry(input: &str) -> IResult<&str, RangeMapEntry> {
    let (input, (destination_start, source_start, length)) = tuple((
        complete::i64,
        preceded(space0, complete::i64),
        preceded(space0, complete::i64),
    ))(input)?;

    Ok((
        input,
        RangeMapEntry {
            destination: Interval::new(destination_start, destination_start + length - 1),
            source: Interval::new(source_start, source_start + length - 1),
        },
    ))
}

fn parse_range_map_entries(input: &str) -> IResult<&str, Vec<RangeMapEntry>> {
    let (input, mut entries) = separated_list1(newline, parse_range_map_entry)(input)?;

    entries.sort_by(|a, b| a.source.cmp(&b.source));

    Ok((input, entries))
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeMap {
    entries: Vec<RangeMapEntry>,
}

impl RangeMap {
    pub fn translate(&self, input: i64) -> i64 {
        for e in self.entries.iter() {
            if let Some(v) = e.translate(input) {
                return v;
            }
        }

        input
    }
}

fn parse_map_mapping(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-to-"), alpha1)(input)
}

fn parse_range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, ((_name, _target), entries)) = separated_pair(
        parse_map_mapping,
        tag(" map:"),
        preceded(multispace1, parse_range_map_entries),
    )(input)?;

    Ok((input, RangeMap { entries }))
}

fn parse_range_maps(input: &str) -> IResult<&str, Vec<RangeMap>> {
    fold_many1(
        preceded(multispace0, parse_range_map),
        Vec::default,
        |mut m, map| {
            m.push(map);
            m
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<RangeMap>)> {
    separated_pair(parse_seeds, multispace1, parse_range_maps)(input)
}

#[derive(Debug, Clone)]
pub struct YouGiveASeedAFertilizer {
    seeds: Vec<i64>,
    range_maps: Vec<RangeMap>,
}

impl YouGiveASeedAFertilizer {
    pub fn lowest_location_number(&self) -> i64 {
        let mut lowest = i64::MAX;
        for seed in self.seeds.iter().copied() {
            let soil = self.range_maps[0].translate(seed);
            let fertilizer = self.range_maps[1].translate(soil);
            let water = self.range_maps[2].translate(fertilizer);
            let light = self.range_maps[3].translate(water);
            let temperature = self.range_maps[4].translate(light);
            let humidity = self.range_maps[5].translate(temperature);
            let location = self.range_maps[6].translate(humidity);
            if location < lowest {
                lowest = location;
            }
        }

        lowest
    }

    pub fn lowest_location_number_range(&self) -> i64 {
        let mut ranges = self
            .seeds
            .iter()
            .tuples()
            .map(|(start, len)| Interval::new(*start, *start + *len -1))
            .collect::<Vec<_>>();

        for map in self.range_maps.iter() {
            let mut next_ranges = Vec::with_capacity(ranges.len());

            'splitter: while let Some(range) = ranges.pop() {
                for entry in map.entries.iter() {
                    match range.partition_by(&entry.source) {
                        IntervalPartition::EntirelyContained { overlap } => {
                            next_ranges.push(overlap.translate(entry.destination.start - entry.source.start));
                            continue 'splitter;
                        },
                        IntervalPartition::RemainderLeft { left, overlap } => {
                            next_ranges.push(left);
                            next_ranges.push(overlap.translate(entry.destination.start - entry.source.start));
                            continue 'splitter;
                        },
                        IntervalPartition::RemainderRight { overlap, right } => {
                            next_ranges.push(overlap.translate(entry.destination.start - entry.source.start));
                            ranges.push(right);
                            continue 'splitter;
                        },
                        IntervalPartition::Bisecting { left, overlap, right } => {
                            next_ranges.push(left);
                            next_ranges.push(overlap.translate(entry.destination.start - entry.source.start));
                            ranges.push(right);
                            continue 'splitter;
                        },
                        _ => { /* nothing */ }
                    }
                }
                // if we're here it means we didn't find _any_ overlaps, so we
                // need to add ourself back to the next iteration
                next_ranges.push(range);
            }

            ranges = next_ranges;
        }

        ranges.iter().map(|c| c.start).min().unwrap_or_default()
    }
}

impl FromStr for YouGiveASeedAFertilizer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (seeds, range_maps)) = parse(s).map_err(|e| e.to_owned())?;

        Ok(Self { seeds, range_maps })
    }
}

impl Problem for YouGiveASeedAFertilizer {
    const DAY: usize = 5;
    const TITLE: &'static str = "you give a seed a fertilizer";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.lowest_location_number())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.lowest_location_number_range())
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
        let solution = YouGiveASeedAFertilizer::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(825516882, 136096660));
    }

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let solution = YouGiveASeedAFertilizer::solve(input).unwrap();
        assert_eq!(solution, Solution::new(35, 46));
    }
}
