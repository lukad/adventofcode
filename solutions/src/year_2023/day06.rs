use aoc::*;
use itertools::Itertools;

#[derive(Debug, Date)]
#[date(year = 2023, day = 6)]
pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> AocResult {
        let input = input.replace("Time:", "").replace("Distance:", "");
        let times = input
            .lines()
            .nth(0)
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let distances = input
            .lines()
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let races = times.into_iter().zip(distances).collect::<Vec<_>>();

        let result = races
            .into_iter()
            .map(|(time, record)| {
                (0..=time)
                    .filter(|hold| hold * (time - hold) > record)
                    .count()
            })
            .reduce(|a, b| a * b)
            .unwrap();

        Ok(Box::new(result))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let input = input.replace("Time:", "").replace("Distance:", "");
        let time = input
            .lines()
            .nth(0)
            .unwrap()
            .split_ascii_whitespace()
            .join("")
            .parse::<usize>()
            .unwrap();
        let distance = input
            .lines()
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .join("")
            .parse::<usize>()
            .unwrap();

        let result = (0..=time)
            .filter(|hold| hold * (time - hold) > distance)
            .count();

        Ok(Box::new(result))
    }
}

#[test]
fn test() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_solution!(Day06.part_one, input, "288");
    assert_solution!(Day06.part_two, input, "71503");
}
