use aoc::*;
use itertools::Itertools;

#[derive(Debug, Date)]
#[date(year = 2023, day = 11)]
pub struct Day11;

fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut galaxies = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }
    galaxies
}

fn expansions(universe: &[(i64, i64)]) -> (Vec<i64>, Vec<i64>) {
    let (min_x, max_x) = universe
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = universe
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();

    let mut expanded_columns = vec![];
    let mut expanded_rows = vec![];

    'outer: for y in min_y..=max_y {
        for x in min_x..=max_x {
            if universe.contains(&(x, y)) {
                continue 'outer;
            }
        }
        expanded_rows.push(y);
    }

    'outer: for x in min_x..=max_x {
        for y in min_y..=max_y {
            if universe.contains(&(x, y)) {
                continue 'outer;
            }
        }
        expanded_columns.push(x);
    }

    (expanded_columns, expanded_rows)
}

fn solve(
    universe: Vec<(i64, i64)>,
    expanded_rows: Vec<i64>,
    expanded_columns: Vec<i64>,
    expand_to: i64,
) -> i64 {
    let mut sum = 0;

    for i in 0..(universe.len() - 1) {
        for j in (i + 1)..universe.len() {
            let (x1, y1) = universe[i];
            let (x2, y2) = universe[j];
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            let mut steps = (x2 - x1).abs() + (y2 - y1).abs();

            for y in expanded_rows.iter() {
                if (y1..y2).contains(y) {
                    steps += expand_to - 1;
                }
            }

            for x in expanded_columns.iter() {
                if (x1..x2).contains(x) {
                    steps += expand_to - 1;
                }
            }

            sum += steps;
        }
    }

    sum
}

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> AocResult {
        let universe = parse(input);
        let (expanded_columns, expanded_rows) = expansions(&universe);
        let sum = solve(universe, expanded_rows, expanded_columns, 2);
        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        #[cfg(test)]
        let expand_to = 10;
        #[cfg(not(test))]
        let expand_to = 1_000_000;

        let universe = parse(input);
        let (expanded_columns, expanded_rows) = expansions(&universe);
        let sum = solve(universe, expanded_rows, expanded_columns, expand_to);
        Ok(Box::new(sum))
    }
}
#[test]
fn test() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_solution!(Day11.part_one, input, "374");
    assert_solution!(Day11.part_two, input, "1030");
}
