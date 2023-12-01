use std::str::FromStr;

use aoc_plumbing::Problem;

fn extract_digit_from_slice(slice: &[u8]) -> u32 {
    if slice[0].is_ascii_digit() {
        (slice[0] - b'0') as u32
    } else if slice.starts_with(b"one") {
        1
    } else if slice.starts_with(b"two") {
        2
    } else if slice.starts_with(b"three") {
        3
    } else if slice.starts_with(b"four") {
        4
    } else if slice.starts_with(b"five") {
        5
    } else if slice.starts_with(b"six") {
        6
    } else if slice.starts_with(b"seven") {
        7
    } else if slice.starts_with(b"eight") {
        8
    } else if slice.starts_with(b"nine") {
        9
    } else {
        u32::MAX
    }
}

#[derive(Debug, Clone)]
pub struct Trebuchet {
    lines: u32,
    letter_lines: u32,
}

impl FromStr for Trebuchet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = 0;
        let mut letter_lines = 0;

        for l in s.trim().split('\n') {
            let digits = l
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<_>>();
            lines += match digits.as_slice() {
                [] => 0,
                [a] => a * 10 + a,
                [a, .., b] => a * 10 + b,
            };

            let mut sum = 0;
            let bytes = l.as_bytes();
            for i in 0..bytes.len() {
                let slice = &bytes[i..];
                let v = extract_digit_from_slice(slice);
                if v != u32::MAX {
                    sum += v * 10;
                    break;
                }
            }
            for i in 0..bytes.len() {
                let slice = &bytes[(bytes.len() - 1 - i)..];
                let v = extract_digit_from_slice(slice);
                if v != u32::MAX {
                    sum += v;
                    break;
                }
            }
            letter_lines += sum;
        }

        Ok(Self {
            lines,
            letter_lines,
        })
    }
}

impl Problem for Trebuchet {
    const DAY: usize = 1;
    const TITLE: &'static str = "trebuchet";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u32;
    type P2 = u32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        // it was easier to solve this as part of the parsing
        Ok(self.lines)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // it was easier to solve this as part of the parsing
        Ok(self.letter_lines)
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
        let solution = Trebuchet::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(54159, 53866));
    }

    #[test]
    fn example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let solution = Trebuchet::solve(input).unwrap();
        assert_eq!(solution, Solution::new(209, 281));
    }
}
