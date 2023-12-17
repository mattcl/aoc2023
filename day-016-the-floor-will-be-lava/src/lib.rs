use std::{ops::BitOrAssign, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::Cardinal, geometry::Location};
use nom::{
    character::complete::{self, newline},
    combinator,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::prelude::*;

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct EnergizedSet {
    rows: Vec<u128>,
}

impl EnergizedSet {
    pub fn new(num_rows: usize) -> Self {
        Self {
            rows: vec![0; num_rows],
        }
    }

    pub fn add(&mut self, location: &Location) {
        self.rows[location.row] |= 1 << location.col;
    }

    pub fn count(&self) -> usize {
        self.rows.iter().map(|r| r.count_ones() as usize).sum()
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.rows[location.row] & 1 << location.col != 0
    }
}

impl BitOrAssign for EnergizedSet {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..self.rows.len() {
            self.rows[i] |= rhs.rows[i];
        }
    }
}

/// Our primary goal is to reduce the time it takes to iterate through one of
/// the beam propagations. This mapping is faster than FxHashSet.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct VisistedMap {
    north: EnergizedSet,
    south: EnergizedSet,
    east: EnergizedSet,
    west: EnergizedSet,
}

impl VisistedMap {
    pub fn new(num_rows: usize) -> Self {
        Self {
            north: EnergizedSet::new(num_rows),
            south: EnergizedSet::new(num_rows),
            east: EnergizedSet::new(num_rows),
            west: EnergizedSet::new(num_rows),
        }
    }

    pub fn add(&mut self, particle: &Particle) {
        match particle.facing {
            Cardinal::North => self.north.add(&particle.location),
            Cardinal::South => self.south.add(&particle.location),
            Cardinal::East => self.east.add(&particle.location),
            Cardinal::West => self.west.add(&particle.location),
        }
    }

    pub fn contains(&mut self, particle: &Particle) -> bool {
        match particle.facing {
            Cardinal::North => self.north.contains(&particle.location),
            Cardinal::South => self.south.contains(&particle.location),
            Cardinal::East => self.east.contains(&particle.location),
            Cardinal::West => self.west.contains(&particle.location),
        }
    }

    pub fn contains_opposite(&mut self, particle: &Particle) -> bool {
        match particle.facing {
            Cardinal::North => self.south.contains(&particle.location),
            Cardinal::South => self.north.contains(&particle.location),
            Cardinal::East => self.west.contains(&particle.location),
            Cardinal::West => self.east.contains(&particle.location),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TheFloorWillBeLava {
    grid: Grid<Tile>,
}

impl TheFloorWillBeLava {
    pub fn propagate(&self, start: Particle) -> EnergizedSet {
        // let mut seen: FxHashSet<Particle> = FxHashSet::default();
        let mut seen = VisistedMap::new(self.grid.height());
        let mut energized = EnergizedSet::new(self.grid.height());

        let mut beams = vec![start];

        while let Some(mut beam) = beams.pop() {
            let tile = self.grid.get(&beam.location).unwrap();
            if seen.contains(&beam) || (*tile == Tile::Empty && seen.contains_opposite(&beam)) {
                continue;
            }
            seen.add(&beam);
            energized.add(&beam.location);

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

        energized
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
            .map(|p| self.propagate(p).count())
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
        Ok(self.propagate(Particle::default()).count())
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
