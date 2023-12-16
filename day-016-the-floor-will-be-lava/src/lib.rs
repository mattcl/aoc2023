use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::Cardinal, geometry::Location};
use nom::{
    character::complete::{self, newline},
    combinator,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Empty,
    MirrorF,
    MirrorB,
    HorizSplit,
    VertSplit,
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    combinator::map(complete::one_of(".|/\\-"), |ch| match ch {
        '.' => Tile::Empty,
        '/' => Tile::MirrorF,
        '\\' => Tile::MirrorB,
        '|' => Tile::VertSplit,
        '-' => Tile::HorizSplit,
        _ => unreachable!(),
    })(input)
}

fn parse_tile_row(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(parse_tile)(input)
}

fn parse_tile_rows(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, parse_tile_row)(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Particle {
    location: Location,
    facing: Cardinal,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            location: Location::new(0, 0),
            facing: Cardinal::East,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TheFloorWillBeLava {
    grid: Grid<Tile>,
}

impl TheFloorWillBeLava {
    pub fn propagate(&self, start: Particle) -> usize {
        let mut seen: FxHashSet<Particle> = FxHashSet::default();
        let mut energized: FxHashSet<Location> = FxHashSet::default();

        let mut beams = vec![start];

        while let Some(mut beam) = beams.pop() {
            if seen.contains(&beam) {
                continue;
            }
            seen.insert(beam);
            energized.insert(beam.location);

            let tile = self.grid.get(&beam.location).unwrap();

            match tile {
                Tile::Empty => {}
                Tile::MirrorF => match beam.facing {
                    Cardinal::North => beam.facing = Cardinal::East,
                    Cardinal::South => beam.facing = Cardinal::West,
                    Cardinal::East => beam.facing = Cardinal::North,
                    Cardinal::West => beam.facing = Cardinal::South,
                },
                Tile::MirrorB => match beam.facing {
                    Cardinal::North => beam.facing = Cardinal::West,
                    Cardinal::South => beam.facing = Cardinal::East,
                    Cardinal::East => beam.facing = Cardinal::South,
                    Cardinal::West => beam.facing = Cardinal::North,
                },
                Tile::HorizSplit => match beam.facing {
                    Cardinal::North | Cardinal::South => {
                        let mut other = beam;
                        other.facing = Cardinal::East;
                        if let Some((next, _)) =
                            self.grid.cardinal_neighbor(&other.location, other.facing)
                        {
                            other.location = next;
                            beams.push(other);
                        }

                        beam.facing = Cardinal::West;
                    }
                    _ => {}
                },
                Tile::VertSplit => match beam.facing {
                    Cardinal::East | Cardinal::West => {
                        let mut other = beam;
                        other.facing = Cardinal::North;
                        if let Some((next, _)) =
                            self.grid.cardinal_neighbor(&other.location, other.facing)
                        {
                            other.location = next;
                            beams.push(other);
                        }

                        beam.facing = Cardinal::South;
                    }
                    _ => {}
                },
            }

            if let Some((next, _)) = self.grid.cardinal_neighbor(&beam.location, beam.facing) {
                beam.location = next;
                beams.push(beam);
            }
        }

        energized.len()
    }

    pub fn propagate_all(&self) -> usize {
        let height = self.grid.height();
        let width = self.grid.width();
        let mut starting_particles = Vec::with_capacity((width + height) * 2);

        for row in 0..height {
            starting_particles.push(Particle {
                location: (row, 0).into(),
                facing: Cardinal::East,
            });
            starting_particles.push(Particle {
                location: (row, width - 1).into(),
                facing: Cardinal::West,
            });
        }

        for col in 0..width {
            starting_particles.push(Particle {
                location: (0, col).into(),
                facing: Cardinal::South,
            });
            starting_particles.push(Particle {
                location: (height - 1, col).into(),
                facing: Cardinal::North,
            });
        }

        starting_particles
            .into_par_iter()
            .map(|p| self.propagate(p))
            .max()
            .unwrap_or_default()
    }
}

impl FromStr for TheFloorWillBeLava {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, tiles) = parse_tile_rows(s).map_err(|e| e.to_owned())?;

        Ok(Self {
            grid: Grid::new(tiles),
        })
    }
}

impl Problem for TheFloorWillBeLava {
    const DAY: usize = 16;
    const TITLE: &'static str = "the floor will be lava";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.propagate(Particle::default()))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.propagate_all())
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
        let solution = TheFloorWillBeLava::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(8249, 8444));
    }

    #[test]
    fn example() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let solution = TheFloorWillBeLava::solve(input).unwrap();
        assert_eq!(solution, Solution::new(46, 51));
    }
}
