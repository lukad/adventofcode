pub use aoc_derive::aoc;

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

pub type Day = i32;
pub type Year = i32;

pub trait Solution {
    fn solve(&self, input: &str) -> i32;
    fn year(&self) -> Year;
    fn day(&self) -> Day;
    fn part(&self) -> Part;
}
