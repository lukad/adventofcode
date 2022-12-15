use std::collections::BTreeSet;

use aoc::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::eof,
    multi::separated_list1,
    sequence::{preceded, terminated},
    Finish, IResult,
};

#[derive(Debug, Date)]
#[date(year = 2022, day = 14)]
pub struct Day14;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> AocResult {
        let (_, paths) = parse_paths(input).finish().unwrap();
        let mut map = Map::from(paths);

        let mut steps = 0;
        while map.simulate_sand((500, 0)) {
            steps += 1;
        }

        Ok(Box::new(steps))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (_, paths) = parse_paths(input).finish().unwrap();
        let mut map = Map::from(paths);
        map.add_floor(-100..1000, map.max_y + 2);

        let mut steps = 0;
        while map.simulate_sand((500, 0)) {
            steps += 1;
        }

        Ok(Box::new(steps))
    }
}

#[derive(Clone)]
struct Map {
    data: BTreeSet<(i32, i32)>,
    max_y: i32,
}

impl Map {
    fn simulate_sand(&mut self, source: (i32, i32)) -> bool {
        let mut pos = source;

        if self.data.contains(&source) {
            return false;
        }

        loop {
            match [
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ]
            .into_iter()
            .find(|candidate| !self.data.contains(candidate))
            {
                Some(new_pos) => {
                    if new_pos.1 >= self.max_y + 20 {
                        return false;
                    }
                    pos = new_pos;
                }
                None => {
                    self.data.insert(pos);
                    return true;
                }
            }
        }
    }

    fn add_floor(&mut self, range: std::ops::Range<i32>, height: i32) {
        for x in range {
            self.data.insert((x, height));
        }
    }
}

impl From<Vec<Vec<(i32, i32)>>> for Map {
    fn from(paths: Vec<Vec<(i32, i32)>>) -> Self {
        let mut data: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut max_y = 0;

        for path in paths.into_iter() {
            for (from, to) in path.into_iter().tuple_windows() {
                let mut from = from;
                let dx = (to.0 - from.0).signum();
                let dy = (to.1 - from.1).signum();
                data.insert(from);
                while from != to {
                    from.0 += dx;
                    from.1 += dy;
                    max_y = max_y.max(from.1);
                    data.insert(from);
                }
            }
        }
        Self { data, max_y }
    }
}

fn parse_point(i: &str) -> IResult<&str, (i32, i32)> {
    let (i, x) = nom::character::complete::i32(i)?;
    let (i, _) = nom::character::complete::char(',')(i)?;
    let (i, y) = nom::character::complete::i32(i)?;
    Ok((i, (x, y)))
}

fn parse_path(i: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(tag(" -> "), parse_point)(i)
}

fn parse_paths(i: &str) -> IResult<&str, Vec<Vec<(i32, i32)>>> {
    terminated(separated_list1(newline, parse_path), preceded(newline, eof))(i)
}

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
    assert_solution!(Day14.part_one, input, "24");
    assert_solution!(Day14.part_two, input, "93");
}
