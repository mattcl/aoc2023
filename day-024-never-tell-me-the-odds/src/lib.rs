use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::Point3D;
use itertools::Itertools;
use nalgebra::{matrix, vector, Vector3};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    combinator,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hail {
    position: Point3D<i64>,
    velocity: Point3D<i64>,
}

impl Hail {
    pub fn intersects_xy(&self, other: &Self) -> bool {
        self.velocity.x * other.velocity.y - other.velocity.x * self.velocity.y != 0
    }

    // | vx1, -vx2 | | s | = | x2 - x1 |
    // | vy1, -vy2 | | t |   | y2 - y1 |
    pub fn intersect_location_xy(&self, other: &Self) -> Option<(f64, f64)> {
        let d = other.velocity.x * self.velocity.y - self.velocity.x * other.velocity.y;

        if d != 0 {
            let x1 = self.position.x as f64;
            let y1 = self.position.y as f64;
            let vx1 = self.velocity.x as f64;
            let vy1 = self.velocity.y as f64;
            let x2 = other.position.x as f64;
            let y2 = other.position.y as f64;
            let vx2 = other.velocity.x as f64;
            let vy2 = other.velocity.y as f64;

            let s = ((y2 - y1) * vx2 - vy2 * (x2 - x1)) / d as f64;
            if s < 0.0 {
                return None;
            }

            let t = ((y2 - y1) * vx1 - vy1 * (x2 - x1)) / d as f64;
            if t < 0.0 {
                return None;
            }

            let x = x1 + s * vx1;
            let y = y1 + s * vy1;

            Some((x, y))
        } else {
            None
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Point3D<i64>> {
    combinator::map(
        tuple((
            complete::i64,
            terminated(complete::char(','), space1),
            complete::i64,
            terminated(complete::char(','), space1),
            complete::i64,
        )),
        |(x, _, y, _, z)| Point3D::new(x, y, z),
    )(input)
}

fn parse_hail(input: &str) -> IResult<&str, Hail> {
    combinator::map(
        separated_pair(parse_point, tag(" @ "), parse_point),
        |(position, velocity)| Hail { position, velocity },
    )(input)
}

fn parse_hailstones(input: &str) -> IResult<&str, Vec<Hail>> {
    separated_list1(newline, parse_hail)(input)
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct VectorHail {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl From<Hail> for VectorHail {
    fn from(value: Hail) -> Self {
        Self {
            position: vector![
                value.position.x as f64,
                value.position.y as f64,
                value.position.z as f64
            ],
            velocity: vector![
                value.velocity.x as f64,
                value.velocity.y as f64,
                value.velocity.z as f64
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct NeverTellMeTheOddsGen<const A: i64, const B: i64> {
    hail: Vec<Hail>,
}

impl<const A: i64, const B: i64> NeverTellMeTheOddsGen<A, B> {
    pub fn intersections(&self) -> usize {
        let lower = A as f64;
        let upper = B as f64;
        let mut count = 0;
        self.hail.iter().tuple_combinations().for_each(|(a, b)| {
            if let Some((ix, iy)) = a.intersect_location_xy(b) {
                if lower <= ix && ix <= upper && lower <= iy && iy <= upper {
                    count += 1;
                }
            }
        });

        count
    }

    pub fn find_rock_origin(&self) -> i64 {
        let h0 = VectorHail::from(self.hail[0]);
        let h1 = VectorHail::from(self.hail[1]);
        let h2 = VectorHail::from(self.hail[2]);

        let top = -h0.position.cross(&h0.velocity) + h1.position.cross(&h1.velocity);
        let bot = -h0.position.cross(&h0.velocity) + h2.position.cross(&h2.velocity);

        let rhs = vector![top[0], top[1], top[2], bot[0], bot[1], bot[2]];

        let ul = h0.velocity.cross_matrix() - h1.velocity.cross_matrix();
        let ll = h0.velocity.cross_matrix() - h2.velocity.cross_matrix();
        let ur = -h0.position.cross_matrix() + h1.position.cross_matrix();
        let lr = -h0.position.cross_matrix() + h2.position.cross_matrix();

        let mat = matrix![ul[(0, 0)], ul[(0, 1)], ul[(0, 2)], ur[(0, 0)], ur[(0, 1)], ur[(0, 2)];
                          ul[(1, 0)], ul[(1, 1)], ul[(1, 2)], ur[(1, 0)], ur[(1, 1)], ur[(1, 2)];
                          ul[(2, 0)], ul[(2, 1)], ul[(2, 2)], ur[(2, 0)], ur[(2, 1)], ur[(2, 2)];
                          ll[(0, 0)], ll[(0, 1)], ll[(0, 2)], lr[(0, 0)], lr[(0, 1)], lr[(0, 2)];
                          ll[(1, 0)], ll[(1, 1)], ll[(1, 2)], lr[(1, 0)], lr[(1, 1)], lr[(1, 2)];
                          ll[(2, 0)], ll[(2, 1)], ll[(2, 2)], lr[(2, 0)], lr[(2, 1)], lr[(2, 2)]];

        let decomp = mat.lu();
        let res = decomp.solve(&rhs).expect("Linear resolution failed.");

        // pray to the gods of floating point, I guess
        res[0].round() as i64 + res[1].round() as i64 + res[2].round() as i64
    }
}

impl<const A: i64, const B: i64> FromStr for NeverTellMeTheOddsGen<A, B> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, hail) = parse_hailstones(s).map_err(|e| e.to_owned())?;
        Ok(Self { hail })
    }
}

impl<const A: i64, const B: i64> Problem for NeverTellMeTheOddsGen<A, B> {
    const DAY: usize = 24;
    const TITLE: &'static str = "never tell me the odds";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.intersections())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.find_rock_origin())
    }
}

pub type NeverTellMeTheOdds = NeverTellMeTheOddsGen<200000000000000, 400000000000000>;

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = NeverTellMeTheOdds::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(20963, 999782576459892));
    }

    #[test]
    fn example() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let solution = NeverTellMeTheOddsGen::<7, 27>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(2, 47));
    }
}
