use aoc::*;

#[derive(Debug, Date)]
#[date(year = 2022, day = 20)]
pub struct Day20;

fn solve(input: &str, key: i64, rounds: usize) -> i64 {
    let mut nums = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * key)
        .enumerate()
        .collect::<Vec<(usize, i64)>>();

    for _ in 0..rounds {
        for original_idx in 0..nums.len() {
            let idx = nums.iter().position(|n| n.0 == original_idx).unwrap();
            let value = nums[idx].1;
            let new_idx = (idx as i64 + value).rem_euclid(nums.len() as i64 - 1) as usize;
            let tmp = nums.remove(idx);
            nums.insert(new_idx, tmp);
        }
    }

    let coord_idx = nums.iter().position(|n| n.1 == 0).unwrap();

    [
        nums[(coord_idx + 1_000) % nums.len()].1,
        nums[(coord_idx + 2_000) % nums.len()].1,
        nums[(coord_idx + 3_000) % nums.len()].1,
    ]
    .into_iter()
    .sum()
}

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> AocResult {
        let result = solve(input, 1, 1);
        Ok(Box::new(result))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let result = solve(input, 811589153, 10);
        Ok(Box::new(result))
    }
}

#[test]
fn test() {
    let input = "1
2
-3
3
-2
0
4";
    assert_solution!(Day20.part_one, input, "3");
    assert_solution!(Day20.part_two, input, "1623178306");
}
