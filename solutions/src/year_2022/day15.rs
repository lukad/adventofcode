use aoc::*;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, newline},
    combinator::eof,
    multi::separated_list1,
    sequence::{preceded, terminated},
    Finish, IResult,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[cfg(not(test))]
const PART_1_Y: i64 = 2000000;
#[cfg(test)]
const PART_1_Y: i64 = 10;
#[cfg(not(test))]
const PART_1_RANGE: std::ops::Range<i64> = (-10000000)..10000000;
#[cfg(test)]
const PART_1_RANGE: std::ops::Range<i64> = (-100)..100;
#[cfg(not(test))]
const PART_2_MAX_HEIGHT: i64 = 4000000;
#[cfg(test)]
const PART_2_MAX_HEIGHT: i64 = 20;

type Pos = (i64, i64);

#[derive(Debug, Date)]
#[date(year = 2022, day = 15)]
pub struct Day15;

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> AocResult {
        let (_, report) = parse_report(input).finish().unwrap();

        let y = PART_1_Y;
        let count = PART_1_RANGE
            .into_par_iter()
            .filter(|&x| {
                let mut covered = true;
                for &((sx, sy), (bx, by)) in report.iter() {
                    if (x, y) == (bx, by) {
                        covered = true;
                        break;
                    }
                    if distance(sx, sy, x, y) <= distance(sx, sy, bx, by) {
                        covered = false;
                        break;
                    }
                }
                !covered
            })
            .count();

        Ok(Box::new(count))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (_, report) = parse_report(input).finish().unwrap();

        let result = (0..=PART_2_MAX_HEIGHT).into_par_iter().find_map_any(|y| {
            let mut x = 0;
            while x <= PART_2_MAX_HEIGHT {
                x += 1;
                if report.iter().all(|&((sx, sy), (bx, by))| {
                    let beacon_distance = distance(sx, sy, bx, by);
                    let source_distance = distance(sx, sy, x, y);
                    if source_distance > beacon_distance {
                        true
                    } else {
                        let dy = (sy - y).abs();
                        x = x.max(sx + (beacon_distance - dy));
                        false
                    }
                }) {
                    return Some(4000000 * x + y);
                }
            }
            None
        });

        result
            .map(|freq| Box::new(freq) as Box<dyn std::fmt::Display>)
            .ok_or_else(|| Error::Other("No tuning frequency found".to_string()))
    }
}

#[inline(always)]
fn distance(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
    (ax - bx).abs() + (ay - by).abs()
}

fn parse_row(i: &str) -> IResult<&str, (Pos, Pos)> {
    let (i, x1) = preceded(tag("Sensor at x="), nom::character::complete::i64)(i)?;
    let (i, y1) = preceded(tag(", y="), nom::character::complete::i64)(i)?;
    let (i, x2) = preceded(
        tag(": closest beacon is at x="),
        nom::character::complete::i64,
    )(i)?;
    let (i, y2) = preceded(tag(", y="), nom::character::complete::i64)(i)?;
    Ok((i, ((x1, y1), (x2, y2))))
}

fn parse_report(i: &str) -> IResult<&str, Vec<(Pos, Pos)>> {
    terminated(
        separated_list1(newline, parse_row),
        preceded(multispace0, eof),
    )(i)
}

#[test]
fn test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
    assert_solution!(Day15.part_one, input, "26");
    assert_solution!(Day15.part_two, input, "56000011");
}
