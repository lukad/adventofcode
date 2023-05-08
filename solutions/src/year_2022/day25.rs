use std::{
    fmt::{Display, Write},
    iter::Sum,
    str::FromStr,
};

use aoc::*;

#[derive(Debug)]
struct Snafu(i64);

impl FromStr for Snafu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = 0;

        for char in s.chars() {
            num *= 5;

            match char {
                '2' => num += 2,
                '1' => num += 1,
                '0' => num += 0,
                '-' => num += -1,
                '=' => num += -2,
                _ => return Err(format!("invalid snafu digit {char}")),
            }
        }

        Ok(Self(num))
    }
}

impl Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        Self(iter.map(|snafu| snafu.0).sum())
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num = self.0;
        let mut digits = vec![];

        while num != 0 {
            match num % 5 {
                4 => {
                    num = (num + 1) / 5;
                    digits.push('-');
                }
                3 => {
                    num = (num + 2) / 5;
                    digits.push('=');
                }
                2 => {
                    num = (num - 2) / 5;
                    digits.push('2');
                }
                1 => {
                    num = (num - 1) / 5;
                    digits.push('1');
                }
                0 => {
                    num /= 5;
                    digits.push('0');
                }
                _ => unreachable!(),
            }
        }

        for digit in digits.into_iter().rev() {
            f.write_char(digit)?;
        }

        Ok(())
    }
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 25)]
pub struct Day25;

impl Solution for Day25 {
    fn part_one(&self, input: &str) -> AocResult {
        let sum: Snafu = input
            .lines()
            .map(|line| line.parse::<Snafu>().unwrap())
            .sum();

        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
    assert_solution!(Day25.part_one, input, "2=-1=0");
}
