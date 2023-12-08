use std::collections::HashMap;

use aoc::*;

const OFFSETS: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn solve(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let mut sum = 0;
    let height = grid.len();
    let width = grid[0].len();

    for y in 0..height {
        let mut num = 0;
        let mut is_part = false;

        for x in 0..width {
            if !grid[y][x].is_ascii_digit() {
                if is_part {
                    is_part = false;
                    sum += num;
                }
                num = 0;
                continue;
            };

            num = num * 10 + (grid[y][x] - b'0') as usize;

            for &(dx, dy) in OFFSETS.iter() {
                match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    (Some(x), Some(y)) if y < height && x < width => {
                        if grid[y][x] != b'.' && !grid[y][x].is_ascii_digit() {
                            is_part = true;
                        }
                    }
                    _ => continue,
                }
            }
        }

        if is_part {
            sum += num;
        }
    }

    sum
}

#[derive(Debug, Date)]
#[date(year = 2023, day = 3)]
pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input)))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let grid = input
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let height = grid.len();
        let width = grid[0].len();

        let mut ratios = HashMap::new();

        for y in 0..height {
            let mut num = 0;
            let mut is_part = false;
            let mut gear_x = 0;
            let mut gear_y = 0;

            for x in 0..width {
                if !grid[y][x].is_ascii_digit() {
                    if is_part {
                        is_part = false;
                        ratios.entry((gear_x, gear_y)).or_insert(vec![]).push(num);
                    }
                    num = 0;
                    continue;
                };

                num = num * 10 + (grid[y][x] - b'0') as usize;

                if !is_part {
                    for &(dx, dy) in OFFSETS.iter() {
                        match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                            (Some(x), Some(y)) if y < height && x < width => {
                                if grid[y][x] == b'*' && !grid[y][x].is_ascii_digit() {
                                    is_part = true;
                                    gear_x = x;
                                    gear_y = y;
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }

            if is_part {
                ratios.entry((gear_x, gear_y)).or_insert(vec![]).push(num);
            }
        }

        let sum: usize = ratios
            .into_values()
            .filter_map(|nums| {
                if nums.len() == 2 {
                    Some(nums[0] * nums[1])
                } else {
                    None
                }
            })
            .sum();

        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_solution!(Day03.part_one, input, "4361");
    assert_solution!(Day03.part_two, input, "467835");
}
