use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::{Cube, Point2D, Point3D};
use nom::{
    character::complete::{self, newline},
    combinator,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Brick {
    cube: Cube<i16>,
}

impl Brick {
    pub fn new(p1: Point3D<i16>, p2: Point3D<i16>) -> Self {
        let cube = Cube::new(p1, p2);
        Self { cube }
    }

    pub fn set_z(&mut self, value: i16) {
        let delta = value - self.cube.start.z;
        self.cube.translate_z(delta);
    }

    pub fn points(&self) -> impl Iterator<Item = Point2D<i16>> {
        let sx = self.cube.start.x.min(self.cube.end.x);
        let ex = self.cube.start.x.max(self.cube.end.x);
        let sy = self.cube.start.y.min(self.cube.end.y);
        let ey = self.cube.start.y.max(self.cube.end.y);
        (sx..=ex).flat_map(move |x| (sy..=ey).map(move |y| Point2D::new(x, y)))
    }
}

fn parse_point(input: &str) -> IResult<&str, Point3D<i16>> {
    combinator::map(
        tuple((
            complete::i16,
            complete::char(','),
            complete::i16,
            complete::char(','),
            complete::i16,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StableEntry {
    index: usize,
    z_height: i16,
}

impl Ord for StableEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .z_height
            .cmp(&self.z_height)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for StableEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct SandSlabs {
    p1: usize,
    p2: usize,
}

impl SandSlabs {
    pub fn settle(bricks: Vec<Brick>) -> (usize, usize) {
        let num_bricks = bricks.len();
        let mut bricks = bricks;
        let mut max_x = 0;
        let mut max_y = 0;

        for b in bricks.iter() {
            if b.cube.start.x > max_x {
                max_x = b.cube.start.x;
            }

            if b.cube.end.x > max_x {
                max_x = b.cube.end.x;
            }

            if b.cube.start.y > max_y {
                max_y = b.cube.start.y;
            }

            if b.cube.end.y > max_y {
                max_y = b.cube.end.y;
            }
        }

        // this is a Vec<Vec<brick index>>
        let mut topology = vec![vec![usize::MAX; max_x as usize + 1]; max_y as usize + 1];
        let mut above: Vec<Vec<usize>> = vec![Vec::default(); bricks.len()];
        let mut below: Vec<Vec<usize>> = vec![Vec::default(); bricks.len()];
        let mut required = vec![false; bricks.len()];

        let mut highest_idxs = Vec::new();
        for i in 0..bricks.len() {
            let brick = &bricks[i];

            let mut highest = 0;

            for Point2D { x, y } in brick.points() {
                let idx = topology[y as usize][x as usize];
                if idx != usize::MAX && bricks[idx].cube.end.z >= highest {
                    if bricks[idx].cube.end.z > highest {
                        highest_idxs.clear();
                    }

                    if highest_idxs.is_empty() || highest_idxs[highest_idxs.len() - 1] != idx {
                        highest_idxs.push(idx);
                    }

                    highest = bricks[idx].cube.end.z;
                }
                topology[y as usize][x as usize] = i;
            }

            if highest_idxs.len() == 1 {
                required[highest_idxs[0]] = true;
            }

            for idx in highest_idxs.drain(..) {
                above[idx].push(i);
                below[i].push(idx);
            }

            bricks[i].set_z(highest + 1);
        }

        let required_idxs = required
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        let p1 = bricks.len() - required_idxs.len();
        let p2 = required_idxs
            .into_par_iter()
            .map(|i| Self::search(i, &above, &below, num_bricks))
            .sum();

        (p1, p2)
    }

    pub fn search(
        start: usize,
        above: &[Vec<usize>],
        below: &[Vec<usize>],
        num_bricks: usize,
    ) -> usize {
        // bfs from the start
        let mut generation = vec![start];
        let mut next = Vec::default();
        let mut removed = vec![false; num_bricks];
        let mut count = 0;
        removed[start] = true;

        while !generation.is_empty() {
            for i in generation.drain(..) {
                for n in above[i].iter() {
                    if removed[*n] {
                        continue;
                    }

                    if below[*n].iter().all(|b| removed[*b]) {
                        removed[*n] = true;
                        count += 1;
                        next.push(*n);
                    }
                }
            }
            std::mem::swap(&mut generation, &mut next);
        }

        count
    }
}

impl FromStr for SandSlabs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mut bricks) = parse_bricks(s).map_err(|e| e.to_owned())?;
        bricks.sort_by(|a, b| a.cube.start.z.cmp(&b.cube.start.z));

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
