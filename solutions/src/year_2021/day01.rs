use aoc::*;

fn solve(input: &str, window_size: usize) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<_>().unwrap())
        .collect::<Vec<usize>>()
        .windows(window_size)
        .fold(0, |acc, window| {
            if window.last().unwrap() > window.first().unwrap() {
                acc + 1
            } else {
                acc
            }
        })
}

#[derive(Debug, Date)]
#[date(year = 2021, day = 1)]
pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input, 2)))
    }

    fn part_two(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input, 3)))
    }
}

#[test]
fn test() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    assert_solution!(Day01.part_one, input, "7");
    assert_solution!(Day01.part_two, input, "5");
}
