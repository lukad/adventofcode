use std::{cmp::Reverse, collections::VecDeque};

use aoc::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        self,
        complete::{multispace0, multispace1},
    },
    combinator::{eof, map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Op,
    test: Test,
    inspections: u64,
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    #[inline(always)]
    fn eval(&self, old: u64) -> usize {
        match old % self.divisor == 0 {
            true => self.if_true,
            false => self.if_false,
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    MulOld,
}

impl Op {
    #[inline(always)]
    fn eval(&self, old: u64) -> u64 {
        match self {
            Self::Add(value) => old + value,
            Self::Mul(value) => old * value,
            Self::MulOld => old * old,
        }
    }
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 11)]
pub struct Day11;

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> AocResult {
        let monkey_business = solve(input, 20, true);

        Ok(Box::new(monkey_business))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let monkey_business = solve(input, 10_000, false);

        Ok(Box::new(monkey_business))
    }
}

fn solve(input: &str, rounds: usize, relief: bool) -> u64 {
    let (_rest, mut monkeys) = parse_monkeys(input).unwrap();

    let modulo: u64 = monkeys.iter().map(|m| m.test.divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let (pass_to, new_item) = {
                    let monkey = &mut monkeys[i];
                    monkey.inspections += 1;

                    let mut item = monkey.operation.eval(item);
                    if relief {
                        item /= 3;
                    }
                    item %= modulo;

                    let pass_to = monkey.test.eval(item);

                    (pass_to, item)
                };

                monkeys[pass_to].items.push_back(new_item);
            }
        }
    }

    let mut inspections: Vec<u64> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_by_key(|&x| Reverse(x));
    inspections.iter().take(2).product()
}

pub fn parse_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    terminated(
        separated_list1(tag("\n\n"), parse_monkey),
        preceded(multispace0, eof),
    )(i)
}

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _id) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(i)?;
    let (i, _) = multispace1(i)?;
    let (i, items) = items(i)?;
    let (i, _) = multispace1(i)?;
    let (i, operation) = operation(i)?;
    let (i, _) = multispace1(i)?;
    let (i, test) = test(i)?;
    Ok((
        i,
        Monkey {
            items,
            operation,
            test,
            inspections: 0,
        },
    ))
}

fn items(i: &str) -> Result<(&str, VecDeque<u64>), nom::Err<nom::error::Error<&str>>> {
    preceded(
        tag("Starting items: "),
        map(
            separated_list1(tag(", "), nom::character::complete::u64),
            |items| items.into(),
        ),
    )(i)
}

fn operation(i: &str) -> IResult<&str, Op> {
    let (i, _) = tag("Operation: new = old")(i)?;
    let (i, _) = multispace0(i)?;
    let add = map(preceded(tag("+ "), character::complete::u64), Op::Add);
    let mul = map(preceded(tag("* "), character::complete::u64), Op::Mul);
    let mul_old = value(Op::MulOld, tag("* old"));
    alt((add, mul, mul_old))(i)
}

fn test(i: &str) -> IResult<&str, Test> {
    let (i, divisor) = preceded(tag("Test: divisible by "), nom::character::complete::u64)(i)?;
    let (i, _) = multispace1(i)?;
    let (i, if_true) = preceded(
        tag("If true: throw to monkey "),
        map(nom::character::complete::u64, |x| x as _),
    )(i)?;
    let (i, _) = multispace1(i)?;
    let (i, if_false) = preceded(
        tag("If false: throw to monkey "),
        map(nom::character::complete::u64, |x| x as _),
    )(i)?;
    Ok((
        i,
        Test {
            divisor,
            if_true,
            if_false,
        },
    ))
}

#[cfg(test)]
mod tests {
    use aoc::*;

    use super::Day11;

    #[test]
    fn test() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        assert_solution!(Day11.part_one, input, "10605");
        assert_solution!(Day11.part_two, input, "2713310158");
    }
}
