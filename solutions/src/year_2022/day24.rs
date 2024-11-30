use std::{collections::VecDeque, ops::Index};

use aoc::*;
use glam::IVec2;
use hashbrown::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Blizzards(Vec<IVec2>),
}

impl Cell {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 24)]
pub struct Day24;

struct Valley {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    start: IVec2,
    end: IVec2,
}

impl Index<IVec2> for Valley {
    type Output = Cell;

    fn index(&self, index: IVec2) -> &Self::Output {
        if index.x < 0 || index.y < 0 {
            return &Cell::Wall;
        }

        self.cells
            .get(index.y as usize)
            .and_then(|row| row.get(index.x as usize))
            .unwrap_or(&Cell::Wall)
    }
}

impl Valley {
    fn step(&mut self) {
        let mut next = vec![vec![Cell::Empty; self.width]; self.height];

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Wall => next[y][x] = Cell::Wall,
                    Cell::Blizzards(blizzards) => {
                        for &blizzard in blizzards.iter() {
                            let mut next_pos = IVec2::new(x as i32, y as i32) + blizzard;
                            if next_pos.x == 0 {
                                next_pos.x = self.width as i32 - 2;
                            } else if next_pos.x == self.width as i32 - 1 {
                                next_pos.x = 1;
                            }
                            if next_pos.y == 0 {
                                next_pos.y = self.height as i32 - 2;
                            } else if next_pos.y == self.height as i32 - 1 {
                                next_pos.y = 1;
                            }
                            let x = &mut next[next_pos.y as usize][next_pos.x as usize];
                            match x {
                                Cell::Empty => *x = Cell::Blizzards(vec![blizzard]),
                                Cell::Blizzards(items) => items.push(blizzard),
                                _ => unreachable!(),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        self.cells = next;
    }
}

fn bfs(valley: &mut Valley, start: IVec2, goal: IVec2) -> usize {
    valley.step();
    let mut queue = VecDeque::from([start]);
    let mut new = HashSet::new();
    let mut steps = 0;

    loop {
        while let Some(pos) = queue.pop_front() {
            if pos == goal {
                return steps;
            }
            for neigbhor in [IVec2::X, IVec2::Y, IVec2::ZERO, IVec2::NEG_X, IVec2::NEG_Y]
                .map(|offset| offset + pos)
            {
                if valley[neigbhor].is_empty() {
                    new.insert(neigbhor);
                }
            }
        }

        queue.extend(new.iter());
        new.clear();
        steps += 1;
        valley.step();
    }
}

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut valley = parse_valley(input);
        let start = valley.start;
        let goal = valley.end;
        let steps = bfs(&mut valley, start, goal);
        Ok(Box::new(steps))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut valley = parse_valley(input);
        let start = valley.start;
        let goal = valley.end;
        let mut steps = bfs(&mut valley, start, goal);
        steps += bfs(&mut valley, goal, start) + 1;
        steps += bfs(&mut valley, start, goal) + 1;
        Ok(Box::new(steps))
    }
}

fn parse_valley(input: &str) -> Valley {
    let mut cells = vec![];
    let mut row = vec![];
    for line in input.lines() {
        for c in line.chars() {
            let cell = match c {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                '^' => Cell::Blizzards(vec![IVec2::NEG_Y]),
                'v' => Cell::Blizzards(vec![IVec2::Y]),
                '<' => Cell::Blizzards(vec![IVec2::NEG_X]),
                '>' => Cell::Blizzards(vec![IVec2::X]),
                _ => panic!("Invalid input character: {c}"),
            };
            row.push(cell);
        }
        cells.push(row);
        row = vec![];
    }

    let width = cells.first().unwrap().len();
    let height = cells.len();
    let start = IVec2::new(1, 0);
    let end = IVec2::new(width as i32 - 2, height as i32 - 1);

    Valley {
        cells,
        width,
        height,
        start,
        end,
    }
}

#[test]
fn test() {
    let input = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#\n";
    assert_solution!(Day24.part_one, input, "18");
    assert_solution!(Day24.part_two, input, "54");
}
