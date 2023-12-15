use std::str::FromStr;

use aoc_plumbing::Problem;

fn axis_sum(pos_counts: &[i64], expansion: usize) -> usize {
    let mut total = 0;
    let mut distance = 0;
    let mut offset = 0;
    let mut global_idx = 0;
    for (idx, count) in pos_counts.iter().enumerate() {
        let expansion_idx = idx + offset;
        if *count == 0 {
            offset += expansion;
        } else {
            for _ in 0..*count {
                distance += global_idx * expansion_idx - total;
                total += expansion_idx;
                global_idx += 1;
            }
        }
    }
    distance
}

#[derive(Debug, Clone)]
pub struct CosmicExpansion {
    single_expansion: usize,
    million_expansion: usize,
}

impl FromStr for CosmicExpansion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines().collect::<Vec<_>>();
        let width = lines[0].len();

        let mut row_counts = Vec::with_capacity(lines.len());
        let mut col_counts = vec![0; width];

        for line in lines {
            let mut row_count = 0;
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    col_counts[col] += 1;
                    row_count += 1;
                }
            }
            row_counts.push(row_count);
        }

        let single_expansion = axis_sum(&row_counts, 1) + axis_sum(&col_counts, 1);
        let million_expansion = axis_sum(&row_counts, 999_999) + axis_sum(&col_counts, 999_999);

        Ok(Self {
            single_expansion,
            million_expansion,
        })
    }
}

impl Problem for CosmicExpansion {
    const DAY: usize = 11;
    const TITLE: &'static str = "cosmic expansion";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.single_expansion)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.million_expansion)
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
        let solution = CosmicExpansion::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(9724940, 569052586852));
    }

    #[test]
    fn example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let solution = CosmicExpansion::solve(input).unwrap();
        assert_eq!(solution, Solution::new(374, 82000210));
    }
}
