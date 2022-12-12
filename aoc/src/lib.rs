use std::str::FromStr;

pub use aoc_derive::Date;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" | "one" => Ok(Self::One),
            "2" | "two" => Ok(Self::Two),
            _ => Err(format!("Unknown part {}", s)),
        }
    }
}

pub type Day = i32;
pub type Year = i32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    NotImplemented,
    Other(String),
}

pub type AocResult = Result<Box<dyn std::fmt::Display>, Error>;

pub trait Date {
    fn year(&self) -> usize;
    fn day(&self) -> usize;
}

pub trait Solution: Sync + std::fmt::Debug + Date {
    fn solve(&self, input: &str, part: Part) -> AocResult {
        match part {
            Part::One => self.part_one(input),
            Part::Two => self.part_two(input),
        }
    }

    #[allow(unused_variables)]
    fn part_one(&self, input: &str) -> AocResult {
        Err(Error::NotImplemented)
    }

    #[allow(unused_variables)]
    fn part_two(&self, input: &str) -> AocResult {
        Err(Error::NotImplemented)
    }
}

#[derive(Debug)]
pub struct NotImplemented;
impl Solution for NotImplemented {}
impl Date for NotImplemented {
    fn year(&self) -> usize {
        0
    }

    fn day(&self) -> usize {
        0
    }
}

#[macro_export]
macro_rules! assert_solution(
    ($solution:ident . $part:ident, $input:expr, $expected:expr) => {
        assert_eq!($solution.$part($input).map(|x| x.to_string()), Ok($expected.to_string()));
    }
);

inventory::collect!(&'static dyn Solution);
