use std::collections::HashMap;

use aoc::aoc;

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
    let mut count = 0;
    let mut cache = HashMap::new();

    for n in fish.into_iter() {
        count += count_fish(n, days, &mut cache);
    }

    count
}

#[aoc(year = 2021, day = 6, part = "one")]
fn solve_2021_06_01(input: &str) -> Box<usize> {
    let fish = parse(input);
    Box::new(solve(fish, 80))
}

#[aoc(year = 2021, day = 6, part = "two")]
fn solve_2021_06_02(input: &str) -> Box<usize> {
    let fish = parse(input);
    Box::new(solve(fish, 256))
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "3,4,3,1,2";
    assert_eq!(
        solve_2021_06_01.solve(input).to_string(),
        "5934".to_string()
    );

    assert_eq!(
        solve_2021_06_02.solve(input).to_string(),
        "26984457539".to_string()
    );
}
