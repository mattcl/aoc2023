use std::str::FromStr;

use aoc_plumbing::Problem;

fn extract_digit_from_slice(slice: &[u8]) -> u32 {
    if slice.starts_with(b"one") || slice.starts_with(b"1") {
        1
    } else if slice.starts_with(b"two") || slice.starts_with(b"2") {
        2
    } else if slice.starts_with(b"three") || slice.starts_with(b"3") {
        3
    } else if slice.starts_with(b"four") || slice.starts_with(b"4") {
        4
    } else if slice.starts_with(b"five") || slice.starts_with(b"5") {
        5
    } else if slice.starts_with(b"six") || slice.starts_with(b"6") {
        6
    } else if slice.starts_with(b"seven") || slice.starts_with(b"7") {
        7
    } else if slice.starts_with(b"eight") || slice.starts_with(b"8") {
        8
    } else if slice.starts_with(b"nine") || slice.starts_with(b"9") {
        9
    } else if slice.starts_with(b"0") {
        0
    } else {
        u32::MAX
    }
}

fn sum_lines(lines: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for line in lines.iter() {
        sum += match line.as_slice() {
            [] => 0,
            [a] => a * 10 + a,
            [a, .., b] => a * 10 + b,
        };
    }
    sum
}

#[derive(Debug, Clone)]
pub struct Trebuchet {
    lines: Vec<Vec<u32>>,
    letter_lines: Vec<Vec<u32>>,
}

impl FromStr for Trebuchet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = Vec::default();
        let mut letter_lines = Vec::default();

        for (idx, l) in s.trim().split('\n').enumerate() {
            lines.push(
                l.chars()
                    .filter_map(|ch| ch.to_digit(10))
                    .collect::<Vec<_>>(),
            );
            letter_lines.push(Vec::new());
            let bytes = l.as_bytes();
            for i in 0..bytes.len() {
                let slice = &bytes[i..];
                let v = extract_digit_from_slice(slice);
                if v != u32::MAX {
                    letter_lines[idx].push(v);
                    break;
                }
            }
            for i in 0..bytes.len() {
                let slice = &bytes[(bytes.len() - 1 - i)..];
                let v = extract_digit_from_slice(slice);
                if v != u32::MAX {
                    letter_lines[idx].push(v);
                    break;
                }
            }
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
        Ok(sum_lines(&self.lines))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(sum_lines(&self.letter_lines))
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
