use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, multispace0, space1},
    combinator,
    multi::fold_many1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    RED,
    GREEN,
    BLUE,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::RED),
            "green" => Ok(Self::GREEN),
            "blue" => Ok(Self::BLUE),
            _ => Err(anyhow!("Unknown color")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity {
    num: u16,
    color: Color,
}

fn parse_quantity(input: &str) -> IResult<&str, Quantity> {
    let (input, (num, color)) = separated_pair(
        complete::u16,
        space1,
        combinator::map_res(alpha1, Color::from_str),
    )(input)?;
    Ok((input, Quantity { num, color }))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set {
    r: u16,
    g: u16,
    b: u16,
}

impl Set {
    pub fn maximums(&self, other: &Set) -> Set {
        Set {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b),
        }
    }

    pub fn power(&self) -> u32 {
        self.r as u32 * self.g as u32 * self.b as u32
    }

    pub fn subset(&self, other: &Set) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
}

fn fold_set(input: &str) -> IResult<&str, Set> {
    fold_many1(
        preceded(alt((tag(", "), tag("; "), tag(": "))), parse_quantity),
        Set::default,
        |mut s: Set, q| {
            match q.color {
                Color::RED => s.r = s.r.max(q.num),
                Color::GREEN => s.g = s.g.max(q.num),
                Color::BLUE => s.b = s.b.max(q.num),
            }
            s
        },
    )(input)
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Game {
    id: u16,
    minimum: Set,
}

impl Game {
    pub fn new(id: u16, minimum: Set) -> Self {
        Self { id, minimum }
    }
    pub fn is_possible(&self, set: &Set) -> bool {
        self.minimum.subset(set)
    }

    pub fn power(&self) -> u32 {
        self.minimum.power()
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    combinator::map(
        tuple((preceded(tag("Game "), complete::u16), fold_set)),
        |(id, set)| Game::new(id, set),
    )(input)
}

// this is another day where the rest of the operations are so fast, that
// solving this in the parsing step makes sense.
fn fold_games(input: &str) -> IResult<&str, (u16, u32)> {
    let constraint = Set {
        r: 12,
        g: 13,
        b: 14,
    };

    fold_many1(
        preceded(multispace0, parse_game),
        || (0_u16, 0_u32),
        move |mut pair, game: Game| {
            if game.minimum.subset(&constraint) {
                pair.0 += game.id;
            }
            pair.1 += game.minimum.power();
            pair
        },
    )(input)
}

#[derive(Debug, Clone)]
pub struct CubeConundrum {
    p1: u16,
    p2: u32,
}

impl FromStr for CubeConundrum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (p1, p2)) = fold_games(s).map_err(|e| e.to_owned())?;
        Ok(Self { p1, p2 })
    }
}

impl Problem for CubeConundrum {
    const DAY: usize = 2;
    const TITLE: &'static str = "cube conundrum";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u16;
    type P2 = u32;

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
        let solution = CubeConundrum::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2476, 54911));
    }

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let solution = CubeConundrum::solve(input).unwrap();
        assert_eq!(solution, Solution::new(8, 2286));
    }
}
