use std::collections::HashSet;

use aoc::*;
use glam::{ivec2, IVec2};

struct Move(IVec2, i32);

#[derive(Debug, Date)]
#[date(year = 2022, day = 9)]
pub struct Day09;

fn parse_move(s: &str) -> Move {
    let (dir, length) = s.split_at(1);
    let length = length.trim().parse().unwrap();
    match dir {
        "U" => Move(ivec2(0, -1), length),
        "D" => Move(ivec2(0, 1), length),
        "L" => Move(ivec2(-1, 0), length),
        "R" => Move(ivec2(1, 0), length),
        _ => panic!("Uknown direction {:?}", dir),
    }
}

fn solve(input: &str, size: usize) -> usize {
    let mut rope = vec![IVec2::ZERO; size];
    let mut visited = HashSet::from([rope[size - 1]]);

    for Move(dir, length) in input.lines().map(parse_move) {
        for _ in 0..length {
            rope[0] += dir;
            for i in 0..size - 1 {
                let a = rope[i];
                let b = rope.get_mut(i + 1).unwrap();
                while (a - *b).as_vec2().length_squared() >= 4.0 {
                    *b += (a - *b).signum();
                    if i == size - 2 {
                        visited.insert(*b);
                    }
                }
            }
        }
    }

    visited.len()
}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input, 2)))
    }
    fn part_two(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input, 10)))
    }
}

#[test]
fn test() {
    let input_1 = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    let input_2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    assert_solution!(Day09.part_one, input_1, "13");
    assert_solution!(Day09.part_two, input_2, "36");
}
