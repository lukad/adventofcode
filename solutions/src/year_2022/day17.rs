use std::time::Duration;

use aoc::*;
use hashbrown::HashSet;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use lazy_static::lazy_static;

lazy_static! {
    static ref PIECES: [Piece; 5] = [
        Piece([
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
        ]),
        Piece([
            [false, false, false, false],
            [false, true, false, false],
            [true, true, true, false],
            [false, true, false, false],
        ]),
        Piece([
            [false, false, false, false],
            [false, false, true, false],
            [false, false, true, false],
            [true, true, true, false],
        ]),
        Piece([
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ]),
        Piece([
            [false, false, false, false],
            [false, false, false, false],
            [true, true, false, false],
            [true, true, false, false],
        ]),
    ];
}

#[derive(Debug)]
struct Piece([[bool; 4]; 4]);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    Left = -1,
    Right = 1,
}

struct Chamber {
    rocks: HashSet<(i64, i64)>,
    top: i64,
    bottom: i64,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rocks: HashSet::new(),
            top: 0,
            bottom: 0,
        }
    }

    fn simulate(&mut self, rocks: usize, moves: impl Iterator<Item = Move> + Clone) {
        let mut pieces = PIECES.iter().cycle();
        let mut moves = moves.cycle();

        let style = ProgressStyle::with_template("{wide_bar} {eta} {pos}/{len}").unwrap();
        for _ in (0..rocks).progress_with_style(style) {
            let piece = pieces.next().unwrap();
            let mut x = 2;
            let mut y = self.top + 3;

            loop {
                let mov = moves.next().unwrap() as i64;

                if !self.piece_collides(piece, x + mov, y, -10) {
                    x += mov;
                }

                if self.piece_collides(piece, x, y - 1, -1) {
                    self.lock_piece(piece, x, y);
                    break;
                }

                y -= 1;
            }

            for y in (self.bottom..=self.top).rev() {
                if (0..7).all(|x| self.rocks.contains(&(x, y))) {
                    for y in self.bottom..=y {
                        for x in 0..7 {
                            self.rocks.remove(&(x, y));
                        }
                    }
                    self.bottom = y;
                    break;
                }
            }
        }
    }

    #[inline(always)]
    fn piece_collides(&self, piece: &Piece, x: i64, y: i64, floor: i64) -> bool {
        for (py, row) in piece.0.iter().rev().enumerate() {
            for (px, cell) in row.iter().enumerate() {
                let world_x = x + px as i64;
                let world_y = y + py as i64;
                if *cell
                    && (world_y == self.bottom + floor
                        || !(0..7).contains(&world_x)
                        || self.rocks.contains(&(world_x, world_y)))
                {
                    return true;
                }
            }
        }
        false
    }

    #[inline(always)]
    fn lock_piece(&mut self, piece: &Piece, x: i64, y: i64) {
        for (py, row) in piece.0.iter().rev().enumerate() {
            for (px, cell) in row.iter().enumerate() {
                if *cell {
                    self.rocks.insert((x + px as i64, y + py as i64));
                    self.top = self.top.max(y + py as i64 + 1);
                }
            }
        }
    }
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 17)]
pub struct Day17;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> AocResult {
        let moves = input.chars().map_while(|c| match c {
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => None,
        });

        let mut chamber = Chamber::new();
        chamber.simulate(2022, moves);

        Ok(Box::new(chamber.top))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let moves = input.chars().map_while(|c| match c {
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => None,
        });

        let mut chamber = Chamber::new();
        chamber.simulate(1000000000000, moves);

        Ok(Box::new(chamber.top))
    }
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_solution!(Day17.part_one, input, "3068");
    // assert_solution!(Day17.part_two, input, "1514285714288");
}
