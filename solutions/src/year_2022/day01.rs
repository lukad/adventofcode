use std::cmp::Reverse;

use aoc::*;

#[derive(Debug, Date)]
#[date(year = 2022, day = 1)]
pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> AocResult {
        let sums = sum_calories(input);
        Ok(Box::new(sums.first().unwrap().to_owned()))
    }
    fn part_two(&self, input: &str) -> AocResult {
        let sums = sum_calories(input);
        Ok(Box::new(sums.iter().take(3).sum::<usize>()))
    }
}

fn sum_calories(input: &str) -> Vec<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sums = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    sums.sort_by_key(|&x| Reverse(x));
    sums
}

#[test]
fn test() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_solution!(Day01.part_one, input, "24000");
    assert_solution!(Day01.part_two, input, "45000");
}
