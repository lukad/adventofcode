use std::collections::HashMap;

use aoc::*;
use pathfinding::prelude::dijkstra;

#[derive(Debug, Date)]
#[date(year = 2022, day = 12)]
pub struct Day12;

#[derive(Debug)]
struct HeightMap {
    start: (i32, i32),
    end: (i32, i32),
    map: HashMap<(i32, i32), i32>,
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let mut map = HashMap::new();

        for (y, line) in s.trim().lines().enumerate() {
            let y = y as i32;
            for (x, c) in line.bytes().enumerate() {
                let x = x as i32;
                let cell = match c {
                    b'S' => 0,
                    b'E' => 27,
                    _ => c as i32 - b'a' as i32 + 1,
                };
                map.insert((x, y), cell);
            }
        }

        assert!(!map.is_empty());

        let start = *map.iter().min_by_key(|(_k, v)| *v).unwrap().0;
        let end = *map.iter().max_by_key(|(_k, v)| *v).unwrap().0;

        Self { map, start, end }
    }
}

impl HeightMap {
    fn shortest_path(
        &self,
        start: &(i32, i32),
        end: &(i32, i32),
    ) -> Option<(Vec<(i32, i32)>, i32)> {
        dijkstra(
            start,
            |&(x, y)| {
                let level = self.map.get(&(x, y)).unwrap();
                [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
                    .into_iter()
                    .filter_map(|p| {
                        self.map.get(&p).and_then(|&h| match h - *level {
                            x if x > 1 => None,
                            _ => Some((p, 1)),
                        })
                    })
            },
            |p| p == end,
        )
    }
}

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> AocResult {
        let heightmap = HeightMap::from(input);

        heightmap
            .shortest_path(&heightmap.start, &heightmap.end)
            .map(|path| Box::new(path.1) as Box<dyn std::fmt::Display>)
            .ok_or_else(|| Error::Other("No path found".into()))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let heightmap = HeightMap::from(input);

        heightmap
            .map
            .iter()
            .filter(|(_pos, height)| **height == 1 || **height == 0)
            .map(|(pos, _)| pos)
            .filter_map(|start| {
                heightmap
                    .shortest_path(start, &heightmap.end)
                    .map(|path| path.1)
            })
            .min()
            .map(|length| Box::new(length) as Box<dyn std::fmt::Display>)
            .ok_or_else(|| Error::Other("No path found".into()))
    }
}

#[test]
fn test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_solution!(Day12.part_one, input, "31");
    assert_solution!(Day12.part_two, input, "29");
}
