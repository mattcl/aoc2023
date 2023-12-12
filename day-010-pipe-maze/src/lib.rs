use std::{fmt::Display, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{
    collections::Grid,
    directions::{BoundedCardinalNeighbors, Cardinal, Direction},
    geometry::Location,
};

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

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE90,
            'J' => Self::NW90,
            '7' => Self::SW90,
            'F' => Self::SE90,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!("Invalid input {}", value),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::NE90 => 'L',
            Self::NW90 => 'J',
            Self::SW90 => '7',
            Self::SE90 => 'F',
            Self::Ground => '.',
            Self::Start => 'S',
        }
        .fmt(f)
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

    pub fn shoelace_right(&self) -> i64 {
        #[allow(clippy::if_same_then_else)]
        if self.cur_tile == Tile::Vertical {
            if self.facing == Cardinal::South {
                self.location.col as i64 - 1
            } else {
                -(self.location.col as i64)
            }
        } else if self.cur_tile == Tile::NW90 && self.facing == Cardinal::North {
            -(self.location.col as i64)
        } else if self.cur_tile == Tile::SW90 && self.facing == Cardinal::West {
            -(self.location.col as i64)
        } else if self.cur_tile == Tile::NE90 && self.facing == Cardinal::East {
            self.location.col as i64 - 1
        } else if self.cur_tile == Tile::SE90 && self.facing == Cardinal::South {
            self.location.col as i64 - 1
        } else {
            0
        }
    }

    pub fn shoelace_left(&self) -> i64 {
        #[allow(clippy::if_same_then_else)]
        if self.cur_tile == Tile::Vertical {
            if self.facing == Cardinal::North {
                self.location.col as i64 - 1
            } else {
                -(self.location.col as i64)
            }
        } else if self.cur_tile == Tile::NW90 && self.facing == Cardinal::West {
            -(self.location.col as i64)
        } else if self.cur_tile == Tile::SW90 && self.facing == Cardinal::South {
            -(self.location.col as i64)
        } else if self.cur_tile == Tile::NE90 && self.facing == Cardinal::North {
            self.location.col as i64 - 1
        } else if self.cur_tile == Tile::SE90 && self.facing == Cardinal::East {
            self.location.col as i64 - 1
        } else {
            0
        }
    }

    pub fn advance(&mut self, maze: &Grid<Tile>) {
        if let Some(next_loc) = self.location.cardinal_neighbor(self.facing) {
            if let Some(tile) = maze.get(&next_loc) {
                self.location = next_loc;
                self.cur_tile = *tile;

                match tile {
                    Tile::SW90 => match self.facing {
                        Cardinal::North => {
                            self.facing = Cardinal::West;
                            self.num_left += 1;
                        }
                        Cardinal::East => {
                            self.facing = Cardinal::South;
                            self.num_right += 1;
                        }
                        _ => {}
                    },
                    Tile::SE90 => match self.facing {
                        Cardinal::North => {
                            self.facing = Cardinal::East;
                            self.num_right += 1;
                        }
                        Cardinal::West => {
                            self.facing = Cardinal::South;
                            self.num_left += 1;
                        }
                        _ => {}
                    },
                    Tile::NE90 => match self.facing {
                        Cardinal::South => {
                            self.facing = Cardinal::East;
                            self.num_left += 1;
                        }
                        Cardinal::West => {
                            self.facing = Cardinal::North;
                            self.num_right += 1;
                        }
                        _ => {}
                    },
                    Tile::NW90 => match self.facing {
                        Cardinal::South => {
                            self.facing = Cardinal::West;
                            self.num_right += 1;
                        }
                        Cardinal::East => {
                            self.facing = Cardinal::North;
                            self.num_left += 1;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
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
        // get the two starting positions
        let actors: Vec<_> = self
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
            .collect();

        // determine what the start _should_ be by using the relative directions
        // between the two actors
        // we know the unwrap is safe because these should be two distinct locations
        let relative = actors[1]
            .location
            .relative_direction_from(&actors[0].location)
            .unwrap();
        let start_tile = match relative {
            Direction::East | Direction::West => Tile::Horizontal,
            Direction::North | Direction::South => Tile::Vertical,
            Direction::NorthWest => Tile::SW90,
            Direction::SouthEast => Tile::NE90,
            Direction::SouthWest => {
                match actors[1]
                    .location
                    .relative_direction_from(&self.start)
                    .unwrap()
                {
                    Direction::West => Tile::NW90,
                    Direction::South => Tile::SE90,
                    _ => unreachable!(
                        "This configuration should not be possible {:?} {:?}",
                        actors, relative
                    ),
                }
            }
            _ => unreachable!(
                "This configuration should not be possible {:?} {:?}",
                actors, relative
            ),
        };

        self.maze.set(&self.start, start_tile).unwrap();

        let mut actor_one = actors[0];

        let mut shoelace_right = actor_one.shoelace_right();
        let mut shoelace_left = actor_one.shoelace_left();

        while actor_one.location != self.start {
            self.steps += 1;
            actor_one.advance(&self.maze);
            shoelace_right += actor_one.shoelace_right();
            shoelace_left += actor_one.shoelace_left();
        }

        // if we just had more right turns than left turns, it means the loop
        // contains things to our right
        if actor_one.num_right > actor_one.num_left {
            self.num_inside = shoelace_right as usize;
        } else {
            self.num_inside = shoelace_left as usize;
        }

        // this actually always will evenly divide
        self.steps /= 2;
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
                        let t = Tile::from(ch);
                        if t == Tile::Start {
                            start = Location::new(row, col);
                        }
                        t
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let maze = Grid::new(tiles);

        let mut s = Self {
            start,
            maze,
            steps: 1,
            num_inside: 0,
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
    // #[ignore]
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
    // #[ignore]
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
