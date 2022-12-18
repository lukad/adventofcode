use std::collections::VecDeque;

use aoc::*;
use glam::IVec3;
use hashbrown::HashSet;
use nom::{
    character::complete::{char, multispace0},
    combinator::iterator,
    IResult,
};

#[inline(always)]
fn neighbors(cube: IVec3) -> impl Iterator<Item = IVec3> {
    [
        cube + IVec3::X,
        cube + IVec3::Y,
        cube + IVec3::Z,
        cube + IVec3::NEG_X,
        cube + IVec3::NEG_Y,
        cube + IVec3::NEG_Z,
    ]
    .into_iter()
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 18)]
pub struct Day18;

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut cubes = HashSet::new();
        for cube in iterator(input, parse_cube).into_iter() {
            cubes.insert(cube);
        }

        let sum: usize = cubes
            .iter()
            .map(|&cube| {
                neighbors(cube)
                    .filter(|neighbor| !cubes.contains(neighbor))
                    .count()
            })
            .sum();

        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut cubes = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;

        for cube in iterator(input, parse_cube).into_iter() {
            max_x = max_x.max(cube.x);
            max_y = max_y.max(cube.y);
            max_z = max_z.max(cube.z);
            cubes.insert(cube);
        }

        let x_range = -1..=max_x + 1;
        let y_range = -1..=max_y + 1;
        let z_range = -1..=max_z + 1;

        let mut water = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_front(IVec3::ZERO);

        while let Some(cube) = queue.pop_back() {
            if x_range.contains(&cube.x) && y_range.contains(&cube.y) && z_range.contains(&cube.z) {
                for neighbor in neighbors(cube) {
                    if !cubes.contains(&neighbor) && !water.contains(&neighbor) {
                        queue.push_front(neighbor);
                        water.insert(neighbor);
                    }
                }
            }
        }

        let sum: usize = cubes
            .iter()
            .map(|&cube| {
                neighbors(cube)
                    .filter(|neighbor| water.contains(neighbor))
                    .count()
            })
            .sum();

        Ok(Box::new(sum))
    }
}

fn parse_cube(i: &str) -> IResult<&str, IVec3> {
    let (i, x) = nom::character::complete::i32(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y) = nom::character::complete::i32(i)?;
    let (i, _) = char(',')(i)?;
    let (i, z) = nom::character::complete::i32(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, IVec3::new(x, y, z)))
}

#[test]
fn test() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    assert_solution!(Day18.part_one, input, "64");
    assert_solution!(Day18.part_two, input, "58");
}
