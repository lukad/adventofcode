use aoc::*;
use nom::{
    character::{complete::multispace1, streaming::char},
    combinator::{iterator, map},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Date)]
#[date(year = 2022, day = 4)]
pub struct Day04;

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> AocResult {
        let count = iterator(input, terminated(pair, multispace1))
            .filter(|[a, b, c, d]| (a >= c && b <= d) || (c >= a && d <= b))
            .count();
        Ok(Box::new(count))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let count = iterator(input, terminated(pair, multispace1))
            .filter(|[a, b, c, d]| a <= d && b >= c)
            .count();
        Ok(Box::new(count))
    }
}

fn pair(i: &str) -> IResult<&str, [i32; 4]> {
    map(
        separated_pair(range, char(','), range),
        |((a, b), (c, d))| [a, b, c, d],
    )(i)
}

fn range(i: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        nom::character::complete::i32,
        char('-'),
        nom::character::complete::i32,
    )(i)
}

#[test]
fn test() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    assert_solution!(Day04.part_one, input, "2");
    assert_solution!(Day04.part_two, input, "4");
}
