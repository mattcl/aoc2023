use std::str::FromStr;

use anyhow::bail;
use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid,
    directions::{BoundedCardinalNeighbors, Cardinal},
    geometry::Location,
};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Vertical,
    Horizontal,
    NE90,
    NW90,
    SW90,
    SE90,
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE90,
            'J' => Self::NW90,
            '7' => Self::SW90,
            'F' => Self::SE90,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => bail!("Invalid tile: {}", value),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Actor {
    location: Location,
    facing: Cardinal,
    cur_tile: Tile,
    num_left: usize,
    num_right: usize,
}

impl Actor {
    pub fn new(location: Location, facing: Cardinal, cur_tile: Tile) -> Self {
        Self {
            location,
            facing,
            cur_tile,
            num_left: 0,
            num_right: 0,
        }
    }

    pub fn advance(&mut self, maze: &Grid<Tile>) {
        if let Some(next_loc) = self.location.cardinal_neighbor(self.facing) {
            if let Some(tile) = maze.get(&next_loc) {
                match (self.facing, tile) {
                    (Cardinal::North, Tile::Vertical)
                    | (Cardinal::East, Tile::Horizontal)
                    | (Cardinal::South, Tile::Vertical)
                    | (Cardinal::West, Tile::Horizontal)
                    | (_, Tile::Start) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                    }
                    (Cardinal::North, Tile::SW90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::West;
                        self.num_left += 1;
                    }
                    (Cardinal::North, Tile::SE90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::East;
                        self.num_right += 1;
                    }
                    (Cardinal::South, Tile::NE90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::East;
                        self.num_left += 1;
                    }
                    (Cardinal::South, Tile::NW90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::West;
                        self.num_right += 1;
                    }
                    (Cardinal::East, Tile::SW90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::South;
                        self.num_right += 1;
                    }
                    (Cardinal::East, Tile::NW90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::North;
                        self.num_left += 1;
                    }
                    (Cardinal::West, Tile::SE90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::South;
                        self.num_left += 1;
                    }
                    (Cardinal::West, Tile::NE90) => {
                        self.location = next_loc;
                        self.cur_tile = *tile;
                        self.facing = Cardinal::North;
                        self.num_right += 1;
                    }
                    _ => unreachable!("should not be possible"),
                }
            }
        }
    }

    pub fn right_locs(&self) -> Vec<Location> {
        let mut out = Vec::default();
        let right_dir = self.facing.right();
        if let Some(right) = self.location.cardinal_neighbor(right_dir) {
            out.push(right);
        }

        match (self.facing, self.cur_tile) {
            (Cardinal::North, Tile::NW90) => {
                if let Some(loc) = self.location.cardinal_neighbor(Cardinal::South) {
                    out.push(loc);
                }
            }
            (Cardinal::South, Tile::SE90) => {
                if let Some(loc) = self.location.cardinal_neighbor(Cardinal::North) {
                    out.push(loc);
                }
            }
            (Cardinal::East, Tile::NE90) => {
                if let Some(loc) = self.location.cardinal_neighbor(Cardinal::West) {
                    out.push(loc);
                }
            }
            (Cardinal::West, Tile::SW90) => {
                if let Some(loc) = self.location.cardinal_neighbor(Cardinal::East) {
                    out.push(loc);
                }
            }
            _ => {}
        }

        out
    }
}

#[derive(Debug, Clone)]
pub struct PipeMaze {
    start: Location,
    maze: Grid<Tile>,
    steps: usize,
    num_inside: usize,
}

impl PipeMaze {
    pub fn process_loop(&mut self) {
        let mut actor_seen: FxHashSet<Location> = FxHashSet::default();
        actor_seen.insert(self.start);

        let actors = self
            .maze
            .cardinal_neighbors(&self.start)
            .filter_map(|(dir, loc, tile)| {
                // we need to find the two that connect to us
                match (dir, tile) {
                    (Cardinal::North, Tile::Vertical) | (Cardinal::South, Tile::Vertical) => {
                        Some(Actor::new(loc, dir, *tile))
                    }
                    (Cardinal::North, Tile::SW90) => Some(Actor::new(loc, Cardinal::West, *tile)),
                    (Cardinal::North, Tile::SE90) => Some(Actor::new(loc, Cardinal::East, *tile)),
                    (Cardinal::South, Tile::NW90) => Some(Actor::new(loc, Cardinal::West, *tile)),
                    (Cardinal::South, Tile::NE90) => Some(Actor::new(loc, Cardinal::East, *tile)),
                    (Cardinal::East, Tile::Horizontal) | (Cardinal::West, Tile::Horizontal) => {
                        Some(Actor::new(loc, dir, *tile))
                    }
                    (Cardinal::East, Tile::SW90) => Some(Actor::new(loc, Cardinal::South, *tile)),
                    (Cardinal::East, Tile::NW90) => Some(Actor::new(loc, Cardinal::North, *tile)),
                    (Cardinal::West, Tile::SE90) => Some(Actor::new(loc, Cardinal::South, *tile)),
                    (Cardinal::West, Tile::NE90) => Some(Actor::new(loc, Cardinal::North, *tile)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(actors.len(), 2);

        let mut actor = actors[0];

        actor_seen.insert(actor.location);

        loop {
            self.steps += 1;
            actor.advance(&self.maze);
            actor_seen.insert(actor.location);

            if actor.location == self.start {
                break;
            }
        }

        let loop_len = actor_seen.len();

        // if we just had more right turns than left turns, it means the loop
        // contains things to our right, so just loop again with ourself
        let mut actor = if actor.num_right > actor.num_left {
            actors[0]
        } else {
            // otherwise, the loop contains things to the right of the other
            // actor, so use it for the next loop
            actors[1]
        };

        let mut working = FxHashSet::default();

        loop {
            for loc in actor.right_locs() {
                if !actor_seen.contains(&loc) && self.maze.get(&loc).is_some() {
                    working.insert(loc);
                }
            }

            if !working.is_empty() {
                self.flood(&working, &mut actor_seen);
                working.clear()
            }

            if actor.location == self.start {
                break;
            }

            actor.advance(&self.maze);
        }

        // this actually always will evenly divide
        self.steps /= 2;

        self.num_inside = actor_seen.len() - loop_len;
    }

    pub fn flood(&self, cur: &FxHashSet<Location>, seen: &mut FxHashSet<Location>) {
        let mut next = FxHashSet::default();
        for loc in cur.iter() {
            if seen.contains(loc) {
                continue;
            }
            seen.insert(*loc);
            next.extend(
                self.maze
                    .cardinal_neighbors(loc)
                    .filter(|(_, next_loc, _)| !seen.contains(next_loc))
                    .map(|(_, next_loc, _)| next_loc),
            );
        }

        if !next.is_empty() {
            self.flood(&next, seen);
        }
    }
}

impl FromStr for PipeMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Location::default();
        let tiles = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        Tile::try_from(ch).map(|t| {
                            if t == Tile::Start {
                                start = Location::new(row, col);
                            }
                            t
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let maze = Grid::new(tiles);

        let mut s = Self {
            start,
            maze,
            steps: 1,
            num_inside: 1,
        };
        s.process_loop();
        Ok(s)
    }
}

impl Problem for PipeMaze {
    const DAY: usize = 10;
    const TITLE: &'static str = "pipe maze";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.steps)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.num_inside)
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
        let solution = PipeMaze::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6860, 343));
    }

    #[test]
    fn part_one_example() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let mut inst = PipeMaze::instance(input).unwrap();
        assert_eq!(inst.part_one().unwrap(), 8);
    }

    #[test]
    fn part_two_example_one() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let mut inst = PipeMaze::instance(input).unwrap();
        inst.part_one().unwrap();
        assert_eq!(inst.part_two().unwrap(), 4);
    }

    #[test]
    fn part_two_example() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let mut inst = PipeMaze::instance(input).unwrap();
        inst.part_one().unwrap();
        assert_eq!(inst.part_two().unwrap(), 8);
    }

    #[test]
    fn part_two_example_harder() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let mut inst = PipeMaze::instance(input).unwrap();
        inst.part_one().unwrap();
        assert_eq!(inst.part_two().unwrap(), 10);
    }
}
