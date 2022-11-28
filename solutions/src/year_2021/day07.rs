use std::convert::identity;

use aoc::*;

fn solve(input: &str, fuel_consumption: fn(isize) -> isize) -> isize {
    let crabs: Vec<isize> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    (*min..*max)
        .map(|point| {
            crabs.iter().fold(0, |acc, crab| {
                let d = (*crab - point).abs();
                acc + fuel_consumption(d)
            })
        })
        .min()
        .unwrap()
}

#[derive(Debug, Date)]
#[date(year = 2021, day = 7)]
pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> aoc::AocResult {
        Ok(Box::new(solve(input, identity)))
    }

    fn part_two(&self, input: &str) -> aoc::AocResult {
        Ok(Box::new(solve(input, |d| d * (d + 1) / 2)))
    }
}

#[test]
fn test() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_solution!(Day07.part_one, input, "37");
    assert_solution!(Day07.part_two, input, "168");
}
