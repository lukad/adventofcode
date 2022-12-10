use std::str::FromStr;

use aoc::*;
use parse_display::{Display, FromStr};

#[derive(Display, Debug, FromStr)]
#[display("move {0} from {1} to {2}")]
pub struct Instruction(usize, usize, usize);

#[derive(Debug, Date)]
#[date(year = 2022, day = 5)]
pub struct Day05;

pub trait Transpose<T> {
    fn transpose(self) -> Vec<Vec<T>>;
}

impl<T> Transpose<T> for Vec<Vec<T>>
where
    T: Clone + std::fmt::Debug,
{
    fn transpose(self) -> Vec<Vec<T>> {
        (0..self[0].len())
            .map(|i| self.iter().map(|el| el[i].clone()).collect::<Vec<T>>())
            .collect()
    }
}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> AocResult {
        let (instructions, mut stacks) = fun_name(input);

        for Instruction(count, from, to) in instructions {
            for _ in 0..count {
                if let Some(item) = stacks[from - 1].pop() {
                    stacks[to - 1].push(item)
                }
            }
        }

        let result: String = stacks.iter().filter_map(|x| x.last()).collect();
        Ok(Box::new(result))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (instructions, mut stacks) = fun_name(input);

        for Instruction(count, from, to) in instructions {
            let insert_position = stacks[to - 1].len();
            for _ in 0..count {
                if let Some(item) = stacks[from - 1].pop() {
                    stacks[to - 1].insert(insert_position, item)
                }
            }
        }

        let result: String = stacks.iter().filter_map(|x| x.last()).collect();
        Ok(Box::new(result))
    }
}

fn fun_name(input: &str) -> (Vec<Instruction>, Vec<Vec<char>>) {
    let lines = input.lines().collect::<Vec<_>>();
    let partition_point = lines.as_slice().partition_point(|line| line.is_empty());
    let (crates, instructions) = lines.split_at(partition_point);
    let stacks = crates[0..crates.len() - 2]
        .iter()
        .map(|line| line.chars().skip(1).step_by(4).collect())
        .collect::<Vec<_>>()
        .transpose()
        .into_iter()
        .map(|stack| stack.into_iter().rev().filter(|c| *c != ' ').collect())
        .collect::<Vec<Vec<_>>>();

    let instructions = instructions
        .iter()
        .filter_map(|line| Instruction::from_str(line).ok())
        .collect();
    (instructions, stacks)
}

#[test]
fn test() {
    let input = "    [D]    \n\
[N] [C]    \n\
[Z] [M] [P]\n\
 1   2   3 \n\
\n\
move 1 from 2 to 1\n\
move 3 from 1 to 3\n\
move 2 from 2 to 1\n\
move 1 from 1 to 2\n";
    assert_solution!(Day05.part_one, input, "CMZ");
    assert_solution!(Day05.part_two, input, "MCD");
}
