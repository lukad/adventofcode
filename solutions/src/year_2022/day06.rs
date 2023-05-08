use aoc::*;
use itertools::Itertools;

#[derive(Debug, Date)]
#[date(year = 2022, day = 6)]
pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> AocResult {
        let chars = input.chars().collect::<Vec<_>>();
        let message_start = solve(&chars, 4);
        Ok(Box::new(message_start))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let chars = input.chars().collect::<Vec<_>>();
        let message_start = solve(&chars, 14);
        Ok(Box::new(message_start))
    }
}

fn solve(chars: &[char], window_size: usize) -> usize {
    for i in 0..chars.len() - 4 {
        if chars[i..i + window_size].iter().all_unique() {
            return i + window_size;
        }
    }
    0
}

#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_solution!(Day06.part_one, input, "7");
    assert_solution!(Day06.part_two, input, "19");
}
