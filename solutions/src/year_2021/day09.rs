use std::collections::{HashMap, HashSet};

use aoc::aoc;

struct HeightMap {
    max_x: isize,
    max_y: isize,
    map: HashMap<(isize, isize), u8>,
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let mut map = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.trim().lines().enumerate() {
            let y = y as isize;
            for (x, c) in line.bytes().enumerate() {
                let x = x as isize;
                map.insert((x, y), c - b'0');
                if x > max_x {
                    max_x = x;
                }
            }
            if y > max_y {
                max_y = y;
            }
        }

        Self { max_x, max_y, map }
    }
}

impl HeightMap {
    fn risk(&self) -> usize {
        let mut risk = 0;

        for point in self.low_points().iter() {
            risk += self.map[point] as usize + 1;
        }

        risk
    }

    fn low_points(&self) -> Vec<(isize, isize)> {
        let mut low_points = vec![];
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let value = &self.map[&(x, y.clone())];
                if [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .all(|(dx, dy)| match self.map.get(&(x + dx, y.clone() + dy)) {
                        Some(other) => other > value,
                        None => true,
                    })
                {
                    low_points.push((x, y));
                }
            }
        }
        low_points
    }

    fn basins(&self) -> usize {
        let mut basins = vec![];
        for point in self.low_points().into_iter() {
            let mut basin = HashSet::new();
            let mut fill = vec![point];
            while let Some((x, y)) = fill.pop() {
                let value = &self.map[&(x, y)];
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let other_coord = (x + dx, y + dy);
                    if let Some(other) = self.map.get(&other_coord) {
                        if other > value && other != &9 && basin.insert(other_coord) {
                            fill.push(other_coord);
                        }
                    }
                }
            }
            basins.push(basin.len() + 1);
        }
        basins.sort();
        basins.into_iter().rev().take(3).product()
    }
}

#[aoc(year = 2021, day = 9, part = "one")]
pub fn solve_2021_09_01(input: &str) -> Box<usize> {
    Box::new(HeightMap::from(input).risk())
}

#[aoc(year = 2021, day = 9, part = "two")]
fn solve_2021_09_02(input: &str) -> Box<i32> {
    Box::new(HeightMap::from(input).basins())
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(solve_2021_09_01.solve(input).to_string(), "15".to_string());
    assert_eq!(
        solve_2021_09_02.solve(input).to_string(),
        "1134".to_string()
    );
}
