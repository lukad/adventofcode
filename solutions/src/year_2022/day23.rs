use std::collections::VecDeque;

use aoc::*;
use glam::IVec2;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Elves = HashSet<IVec2>;

fn parse_elves(input: &str) -> Elves {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| IVec2::new(x as i32, y as i32))
        })
        .collect()
}

fn count_neighbors(elves: &Elves, elf: &IVec2) -> usize {
    [
        IVec2::new(1, 0),
        IVec2::new(1, 1),
        IVec2::new(0, 1),
        IVec2::new(-1, 1),
        IVec2::new(-1, 0),
        IVec2::new(-1, -1),
        IVec2::new(0, -1),
        IVec2::new(1, -1),
    ]
    .into_iter()
    .map(|dir| dir + *elf)
    .filter(|other| elves.contains(other))
    .count()
}

fn simulate(elves: &mut Elves, max_rounds: usize) -> usize {
    let mut moves: VecDeque<(IVec2, [IVec2; 3])> = VecDeque::new();
    moves.extend(
        [
            (
                IVec2::new(0, -1),
                [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
            ),
            (
                IVec2::new(0, 1),
                [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
            ),
            (
                IVec2::new(-1, 0),
                [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
            ),
            (
                IVec2::new(1, 0),
                [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
            ),
        ]
        .iter(),
    );

    let mut rounds = 0;
    for _ in 0..max_rounds {
        if rounds == max_rounds {
            break;
        }

        let moving_elves = elves
            .iter()
            .filter(|elf| count_neighbors(elves, elf) != 0)
            .cloned()
            .collect::<Vec<_>>();

        let mut proposed_moves = HashMap::new();

        for elf in moving_elves {
            for (mov, checks) in moves.iter() {
                if checks.iter().any(|check| elves.contains(&(elf + *check))) {
                    continue;
                }

                let dest = elf + *mov;
                if !elves.contains(&dest) {
                    proposed_moves.insert(elf, dest);
                    break;
                }
            }
        }

        let dupes = proposed_moves.values().duplicates().collect_vec();

        let proposed_moves = proposed_moves
            .iter()
            .filter(|(_from, to)| !dupes.contains(to))
            .collect::<HashMap<&IVec2, &IVec2>>();

        let proposed_moves_count = proposed_moves.len();

        if proposed_moves_count == 0 {
            return rounds + 1;
        }

        for (from, to) in proposed_moves.into_iter() {
            elves.remove(from);
            elves.insert(*to);
        }

        let first_move = moves.pop_front().unwrap();
        moves.push_back(first_move);

        rounds += 1;
    }

    let (left, right) = elves
        .iter()
        .minmax_by_key(|&&elf| elf.x)
        .into_option()
        .unwrap();

    let (top, bottom) = elves
        .iter()
        .minmax_by_key(|&&elf| elf.y)
        .into_option()
        .unwrap();

    let a = right.x - left.x + 1;
    let b = bottom.y - top.y + 1;

    a as usize * b as usize - elves.len()
}

#[allow(unused)]
fn show_elves(elves: &Elves) {
    let (left, right) = elves
        .iter()
        .minmax_by_key(|&&elf| elf.x)
        .into_option()
        .unwrap();

    let (top, bottom) = elves
        .iter()
        .minmax_by_key(|&&elf| elf.y)
        .into_option()
        .unwrap();

    for y in top.y..=bottom.y {
        print!("{y}\t");
        for x in left.x..=right.x {
            if elves.contains(&IVec2::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    println!();
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 23)]
pub struct Day23;

impl Solution for Day23 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut elves = parse_elves(input);
        Ok(Box::new(simulate(&mut elves, 10)))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut elves = parse_elves(input);
        Ok(Box::new(simulate(&mut elves, usize::MAX)))
    }
}

#[test]
fn test() {
    let input = "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..\n";
    assert_solution!(Day23.part_one, input, "110");
    assert_solution!(Day23.part_two, input, "20");
}
