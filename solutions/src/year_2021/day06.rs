use std::collections::HashMap;

use aoc::*;

fn count_fish(fish: u8, days: usize, cache: &mut HashMap<(u8, usize), usize>) -> usize {
    let key = (fish, days);
    if let Some(count) = cache.get(&key) {
        return *count;
    }

    let mut count = 1;
    let mut fish = fish;

    for day in 0..days {
        match fish {
            0 => {
                fish = 6;
                count += count_fish(9, days - day, cache);
            }
            _ => fish -= 1,
        }
    }

    cache.insert(key, count);
    count
}

fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<_>().unwrap())
        .collect()
}

fn solve(fish: Vec<u8>, days: usize) -> usize {
    let mut cache = HashMap::new();

    fish.into_iter()
        .map(|fish| count_fish(fish, days, &mut cache))
        .sum()
}

#[derive(Debug, Date)]
#[date(year = 2021, day = 6)]
pub struct Day06;

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(parse(input), 80)))
    }

    fn part_two(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(parse(input), 256)))
    }
}

#[test]
fn test() {
    let input = "3,4,3,1,2";
    assert_solution!(Day06.part_one, input, "5934");
    assert_solution!(Day06.part_two, input, "26984457539");
}
