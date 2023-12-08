use aoc::*;

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        Some(c.to_digit(10).unwrap() as usize)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut nums = vec![];
            for i in 0..line.len() {
                let first_char = line.chars().nth(i).unwrap();
                if first_char.is_ascii_digit() && first_char != '0' {
                    nums.push(first_char.to_digit(10).unwrap() as usize);
                } else {
                    let rest = &line[i..];
                    if rest.starts_with("one") {
                        nums.push(1);
                    } else if rest.starts_with("two") {
                        nums.push(2);
                    } else if rest.starts_with("three") {
                        nums.push(3);
                    } else if rest.starts_with("four") {
                        nums.push(4);
                    } else if rest.starts_with("five") {
                        nums.push(5);
                    } else if rest.starts_with("six") {
                        nums.push(6);
                    } else if rest.starts_with("seven") {
                        nums.push(7);
                    } else if rest.starts_with("eight") {
                        nums.push(8);
                    } else if rest.starts_with("nine") {
                        nums.push(9);
                    }
                }
            }
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

#[derive(Debug, Date)]
#[date(year = 2023, day = 1)]
pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input)))
    }

    fn part_two(&self, input: &str) -> AocResult {
        Ok(Box::new(solve2(input)))
    }
}

#[test]
fn test() {
    let input1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let input2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_solution!(Day01.part_one, input1, "142");
    assert_solution!(Day01.part_two, input2, "281");
}
