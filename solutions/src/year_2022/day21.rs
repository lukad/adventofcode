use aoc::*;
use hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::{iterator, map},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Date)]
#[date(year = 2022, day = 21)]
pub struct Day21;

impl Solution for Day21 {
    fn part_one(&self, input: &str) -> AocResult {
        let monkeys = iterator(input, parse_row).collect::<HashMap<_, _>>();

        let a = eval("root", &monkeys);

        Ok(Box::new(a))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut monkeys = iterator(input, parse_row).collect::<HashMap<_, _>>();

        let root_job = match *monkeys.get("root").unwrap() {
            Job::Num(_) => panic!(),
            Job::Add(l, r) | Job::Sub(l, r) | Job::Mul(l, r) | Job::Div(l, r) => Job::Sub(r, l),
        };
        monkeys.insert("root", root_job);

        let mut lower = 0;
        let mut upper = 0;
        let mut human = 1;

        loop {
            monkeys.insert("humn", Job::Num(human));
            let root = eval("root", &monkeys);

            dbg!((upper, lower, human, root));
            if root == 0 {
                return Ok(Box::new(human - 1));
            }

            if root < 0 {
                lower = human;
            } else {
                upper = human;
            }

            if lower == 0 || upper == 0 {
                human *= 2;
            } else {
                human = (lower + upper) / 2;
            }
        }
    }
}

fn eval(name: &str, monkeys: &Monkeys) -> i64 {
    match *monkeys.get(name).unwrap() {
        Job::Num(i) => i,
        Job::Add(l, r) => eval(l, monkeys) + eval(r, monkeys),
        Job::Sub(l, r) => eval(l, monkeys) - eval(r, monkeys),
        Job::Div(l, r) => eval(l, monkeys) / eval(r, monkeys),
        Job::Mul(l, r) => eval(l, monkeys) * eval(r, monkeys),
    }
}

type Monkeys<'a> = HashMap<&'a str, Job<'a>>;

#[derive(Debug)]
enum Job<'a> {
    Num(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn parse_job(i: &str) -> IResult<&str, Job<'_>> {
    alt((
        map(
            preceded(multispace0, nom::character::complete::i64),
            Job::Num,
        ),
        map(
            tuple((
                preceded(multispace0, alpha1),
                tag(" +"),
                preceded(multispace0, alpha1),
            )),
            |(l, _, r)| Job::Add(l, r),
        ),
        map(
            tuple((
                preceded(multispace0, alpha1),
                tag(" -"),
                preceded(multispace0, alpha1),
            )),
            |(l, _, r)| Job::Sub(l, r),
        ),
        map(
            tuple((
                preceded(multispace0, alpha1),
                tag(" *"),
                preceded(multispace0, alpha1),
            )),
            |(l, _, r)| Job::Mul(l, r),
        ),
        map(
            tuple((
                preceded(multispace0, alpha1),
                tag(" /"),
                preceded(multispace0, alpha1),
            )),
            |(l, _, r)| Job::Div(l, r),
        ),
    ))(i)
}

fn parse_row(i: &str) -> IResult<&str, (&str, Job<'_>)> {
    let (i, name) = alpha1(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, job) = parse_job(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, (name, job)))
}

#[test]
fn test() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    assert_solution!(Day21.part_one, input, "152");
    assert_solution!(Day21.part_two, input, "301");
}
