use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use aoc::*;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Debug, Date)]
#[date(year = 2023, day = 10)]
pub struct Day10;

#[derive(Display, FromStr, PartialEq)]
enum Pipe {
    #[display("|")]
    Vertical,
    #[display("-")]
    Horizontal,
    #[display("L")]
    NorthEast,
    #[display("J")]
    NorthWest,
    #[display("7")]
    SouthWest,
    #[display("F")]
    SouthEast,
    #[display("S")]
    Start,
}

fn parse(input: &str) -> (BTreeMap<(i32, i32), Pipe>, (i32, i32)) {
    let mut grid = BTreeMap::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let c = Pipe::from_str(&c.to_string()).unwrap();
            if c == Pipe::Start {
                start = (x as i32, y as i32);
            }
            grid.insert((x as i32, y as i32), c);
        }
    }

    (grid, start)
}

fn find_path(start: (i32, i32), grid: &BTreeMap<(i32, i32), Pipe>) -> Vec<(i32, i32)> {
    let mut dir = (0, 1);
    let mut current = (start.0 + dir.0, start.1 + dir.1);
    let mut path = vec![start];

    while current != start {
        path.push(current);

        dir = match (dir, grid.get(&current).unwrap()) {
            ((0, 1), Pipe::Vertical) => (0, 1),
            ((0, -1), Pipe::Vertical) => (0, -1),
            ((1, 0), Pipe::Horizontal) => (1, 0),
            ((-1, 0), Pipe::Horizontal) => (-1, 0),
            ((0, 1), Pipe::NorthEast) => (1, 0),
            ((0, 1), Pipe::NorthWest) => (-1, 0),
            ((0, -1), Pipe::SouthEast) => (1, 0),
            ((0, -1), Pipe::SouthWest) => (-1, 0),
            ((1, 0), Pipe::NorthEast) => (0, -1),
            ((1, 0), Pipe::SouthEast) => (0, 1),
            ((1, 0), Pipe::NorthWest) => (0, -1),
            ((1, 0), Pipe::SouthWest) => (0, 1),
            ((-1, 0), Pipe::NorthEast) => (0, -1),
            ((-1, 0), Pipe::SouthEast) => (0, 1),
            ((-1, 0), Pipe::NorthWest) => (0, -1),
            ((-1, 0), Pipe::SouthWest) => (0, 1),
            _ => panic!(),
        };

        current = (current.0 + dir.0, current.1 + dir.1);
    }

    path
}

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> AocResult {
        let (grid, start) = parse(input);
        let path = find_path(start, &grid);
        Ok(Box::new(path.len() / 2))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (grid, start) = parse(input);
        let path = find_path(start, &grid);
        let path: HashSet<(i32, i32)> = HashSet::from_iter(path);

        let (min_x, max_x) = path.iter().map(|(x, _)| *x).minmax().into_option().unwrap();
        let (min_y, max_y) = path.iter().map(|(_, y)| *y).minmax().into_option().unwrap();

        let mut count = 0;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if path.contains(&(x, y)) {
                    continue;
                }

                let mut crossings = 0;
                let (mut x2, mut y2) = (x, y);
                while x2 <= max_x && y2 <= max_y {
                    if path.contains(&(x2, y2)) {
                        let c = grid.get(&(x2, y2)).unwrap();
                        if c != &Pipe::NorthEast && c != &Pipe::SouthWest {
                            crossings += 1;
                        }
                    }
                    x2 += 1;
                    y2 += 1;
                }

                if crossings % 2 == 1 {
                    count += 1;
                }
            }
        }

        Ok(Box::new(count))
    }
}

#[test]
fn test() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
.";
    let input2 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    assert_solution!(Day10.part_one, input, "8");
    assert_solution!(Day10.part_two, input2, "10");
}
