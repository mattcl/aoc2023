use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{directions::Relative, geometry::Point2D};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{self, newline, one_of},
    combinator::{self, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn from_hex(input: &str) -> Result<i64, std::num::ParseIntError> {
    i64::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_value(input: &str) -> IResult<&str, i64> {
    map_res(take_while_m_n(5, 5, is_hex_digit), from_hex)(input)
}

fn hex_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("#")(input)?;
    let (input, (amount, dir)) = tuple((hex_value, complete::u8))(input)?;

    let direction = match dir {
        0 => Relative::Right,
        1 => Relative::Down,
        2 => Relative::Left,
        3 => Relative::Up,
        _ => unreachable!(),
    };

    Ok((input, Instruction { direction, amount }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
    direction: Relative,
    amount: i64,
}

fn parse_instruction(input: &str) -> IResult<&str, (Instruction, Instruction)> {
    combinator::map(
        tuple((
            map_res(one_of("RDLU"), Relative::try_from),
            preceded(complete::char(' '), complete::i64),
            delimited(tag(" ("), hex_instruction, complete::char(')')),
        )),
        |(direction, amount, hex_instruction)| (Instruction { direction, amount }, hex_instruction),
    )(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<(Instruction, Instruction)>> {
    separated_list1(newline, parse_instruction)(input)
}

#[derive(Debug, Clone)]
pub struct LavaductLagoon {
    instructions: Vec<(Instruction, Instruction)>,
}

impl LavaductLagoon {
    pub fn make_veritices<'a, I: Iterator<Item = &'a Instruction>>(
        &self,
        iter: I,
    ) -> (i64, Vec<Point2D<i64>>) {
        let mut verticies: Vec<Point2D<i64>> = Vec::with_capacity(self.instructions.len() + 1);
        let mut cur: Point2D<i64> = Point2D::default();

        let mut perimeter = 0;
        for inst in iter {
            match inst.direction {
                Relative::Up => cur.y += inst.amount,
                Relative::Down => cur.y -= inst.amount,
                Relative::Left => cur.x -= inst.amount,
                Relative::Right => cur.x += inst.amount,
                _ => unreachable!(),
            }
            verticies.push(cur);
            perimeter += inst.amount;
        }

        (perimeter, verticies)
    }

    pub fn dig<'a, I: Iterator<Item = &'a Instruction>>(&self, iter: I) -> i64 {
        let (perimeter, verticies) = self.make_veritices(iter);
        let area = self.area_from_verticies(&verticies);

        let inside = area - perimeter / 2 + 1;
        perimeter + inside
    }

    // shoelace
    pub fn area_from_verticies(&self, verticies: &[Point2D<i64>]) -> i64 {
        verticies
            .iter()
            .tuple_windows()
            .map(|(a, b)| a.y * b.x - a.x * b.y)
            .sum::<i64>()
            .abs()
            / 2
    }
}

impl FromStr for LavaductLagoon {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, instructions) = parse_instructions(s).map_err(|e| e.to_owned())?;
        Ok(Self { instructions })
    }
}

impl Problem for LavaductLagoon {
    const DAY: usize = 18;
    const TITLE: &'static str = "lavaduct lagoon";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.dig(self.instructions.iter().map(|(a, _)| a)))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.dig(self.instructions.iter().map(|(_, b)| b)))
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
        let solution = LavaductLagoon::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(39194, 78242031808225));
    }

    #[test]
    fn example() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let solution = LavaductLagoon::solve(input).unwrap();
        assert_eq!(solution, Solution::new(62, 952408144115));
    }
}
