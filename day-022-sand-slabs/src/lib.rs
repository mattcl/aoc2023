use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::{Intersect, Point3D, Rectangle};
use itertools::Itertools;
use nom::{
    character::complete::{self, newline},
    combinator,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use rustc_hash::FxHashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Brick {
    start: Point3D<i64>,
    end: Point3D<i64>,
    horiz_rect: Rectangle<i64>,
}

impl Brick {
    pub fn new(p1: Point3D<i64>, p2: Point3D<i64>) -> Self {
        let start = p1.min(p2);
        let end = p2.max(p1);

        let horiz_rect = Rectangle::from_raw(start.x, start.y, end.x, end.y);

        Self {
            start,
            end,
            horiz_rect,
        }
    }

    pub fn set_z(&mut self, value: i64) {
        let delta = self.start.z - value;
        self.start.z = value;
        self.end.z -= delta;
    }

    pub fn z_collision_with(&self, other: &Self) -> bool {
        self.horiz_rect.intersection(&other.horiz_rect).is_some()
    }
}

fn parse_point(input: &str) -> IResult<&str, Point3D<i64>> {
    combinator::map(
        tuple((
            complete::i64,
            complete::char(','),
            complete::i64,
            complete::char(','),
            complete::i64,
        )),
        |(x, _, y, _, z)| Point3D::new(x, y, z),
    )(input)
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    combinator::map(
        separated_pair(parse_point, complete::char('~'), parse_point),
        |(start, end)| Brick::new(start, end),
    )(input)
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(newline, parse_brick)(input)
}

#[derive(Debug, Clone)]
pub struct SandSlabs {
    p1: usize,
    p2: usize,
}

impl SandSlabs {
    pub fn settle(bricks: Vec<Brick>) -> (usize, usize) {
        let mut bricks = bricks;
        let mut stable_bricks: Vec<usize> = Vec::with_capacity(bricks.len());
        let mut above: Vec<Vec<usize>> = vec![Vec::default(); bricks.len()];
        let mut below: Vec<Vec<usize>> = vec![Vec::default(); bricks.len()];

        for i in 0..bricks.len() {
            let brick = bricks[i];

            if brick.start.z > 1 {
                let mut prev = -1;
                for (colliding_height, colliding_brick) in stable_bricks
                    .iter()
                    // on average, we're hoping to be able to set prev sooner to
                    // avoid having to do the more expensive collision checking
                    .rev()
                    .filter_map(|b| {
                        let cur_b = bricks[*b];
                        let next_z = cur_b.end.z + 1;
                        if next_z >= prev && cur_b.z_collision_with(&brick) {
                            if next_z > prev {
                                prev = next_z;
                            }
                            Some((next_z, *b))
                        } else {
                            None
                        }
                    })
                    .sorted_by(|a, b| b.0.cmp(&a.0))
                {
                    if prev == colliding_height {
                        above[colliding_brick].push(i);
                        below[i].push(colliding_brick);
                    } else {
                        break;
                    }
                }

                if prev == -1 {
                    bricks[i].set_z(1);
                } else {
                    bricks[i].set_z(prev);
                }
            }

            stable_bricks.push(i);
        }

        let required = FxHashSet::from_iter(below.iter().filter(|b| b.len() == 1).map(|v| v[0]));

        let p1 = bricks.len() - required.len();

        let p2 = required
            .into_iter()
            .map(|i| Self::search(i, &above, &below))
            .sum();

        (p1, p2)
    }

    pub fn search(start: usize, above: &[Vec<usize>], below: &[Vec<usize>]) -> usize {
        // bfs from the start
        let mut generation = vec![start];
        let mut next = Vec::default();
        let mut removed = FxHashSet::default();

        while !generation.is_empty() {
            for i in generation.iter() {
                removed.insert(*i);
            }

            for i in generation.drain(..) {
                next.extend(
                    above[i]
                        .iter()
                        .filter(|c| below[**c].iter().all(|b| removed.contains(b))),
                );
            }
            std::mem::swap(&mut generation, &mut next);
        }

        removed.len() - 1
    }
}

impl FromStr for SandSlabs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mut bricks) = parse_bricks(s).map_err(|e| e.to_owned())?;
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));

        let (p1, p2) = Self::settle(bricks);
        Ok(Self { p1, p2 })
    }
}

impl Problem for SandSlabs {
    const DAY: usize = 22;
    const TITLE: &'static str = "sand slabs";
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
        let solution = SandSlabs::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(441, 80778));
    }

    #[test]
    fn example() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let solution = SandSlabs::solve(input).unwrap();
        assert_eq!(solution, Solution::new(5, 7));
    }
}
