use std::{fmt::Display, str::FromStr};

use serde::Serialize;

/// This struct enables printing a given solution in either plaintext or JSON,
/// depending on the presence of the `AOC_OUTPUT_JSON` ENV var. Its main purpose
/// is to standardize the output for consuption by the CI system.
///
/// # Usage
///
/// ```
/// use serde_json;
/// use aoc_plumbing::Solution;
/// let s = Solution::new("hello world", 12345);
/// println!("{}", s);
///
/// assert_eq!(
///     s.to_string(),
///     "part 1: hello world\npart 2: 12345"
/// );
/// assert_eq!(
///     serde_json::to_string(&s).unwrap(),
///     "{\"part_one\":\"hello world\",\"part_two\":12345}".to_string()
/// );
/// ```
#[derive(Debug, Serialize, PartialEq)]
pub struct Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    pub part_one: T,
    pub part_two: G,
}

/// The default implementation of `Solution` is as follows:
/// ```
/// use aoc_plumbing::Solution;
/// let default = Solution::default();
/// let expected = Solution::new("not implemented", "not implemented");
///
/// assert_eq!(default, expected);
/// ```
impl Default for Solution<&str, &str> {
    fn default() -> Self {
        Solution::new("not implemented", "not implemented")
    }
}

impl<T, G> Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    pub fn new(part_one: T, part_two: G) -> Self {
        Self { part_one, part_two }
    }
}

impl<T, G> Display for Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "part 1: {}\npart 2: {}", self.part_one, self.part_two)
    }
}

impl<T, G> From<(T, G)> for Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    fn from(value: (T, G)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub trait Problem: FromStr {
    const DAY: usize;
    const TITLE: &'static str;
    const README: &'static str;

    type ProblemError: Send + Sync + From<<Self as FromStr>::Err> + 'static;
    type P1: Display + Serialize + PartialEq;
    type P2: Display + Serialize + PartialEq;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError>;
    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError>;

    fn instance(raw_input: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::from_str(raw_input)
    }

    fn solve(raw_input: &str) -> Result<Solution<Self::P1, Self::P2>, Self::ProblemError> {
        let mut inst = Self::instance(raw_input)?;
        Ok(Solution::new(inst.part_one()?, inst.part_two()?))
    }

    fn problem_label() -> String {
        format!(
            "{:03} {}",
            <Self as Problem>::padded_day(),
            <Self as Problem>::TITLE
        )
    }

    fn padded_day() -> String {
        format!("{:03}", <Self as Problem>::DAY)
    }

    fn long_description() -> String {
        format!(
            "{} {}",
            <Self as Problem>::padded_day(),
            <Self as Problem>::README
        )
    }
}
