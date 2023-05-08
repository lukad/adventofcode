use aoc::*;
use hashbrown::{HashMap, HashSet};
use lazy_static::lazy_static;

type Pos = (i64, i64);
type Piece = HashSet<Pos>;

lazy_static! {
    static ref PIECES: [Piece; 5] = [
        HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
        HashSet::from([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        HashSet::from([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
        HashSet::from([(0, 0), (1, 0), (0, 1), (1, 1)]),
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Move {
    Left = -1,
    Right = 1,
}

struct Chamber {
    rocks: HashSet<(i64, i64)>,
    height: i64,
    piece_id: usize,
    mov_id: usize,
}

impl Chamber {
    fn new() -> Self {
        let mut rocks = HashSet::new();
        rocks.extend((0..7).map(|x| (x, 0)));
        Self {
            rocks,
            height: 0,
            piece_id: 0,
            mov_id: 0,
        }
    }

    fn simulate<F>(mut self, moves: impl Iterator<Item = Move> + Clone, mut callback: F) -> i64
    where
        F: FnMut(&mut Self, usize) -> bool,
    {
        let mut pieces = PIECES.iter().enumerate().cycle();
        let mut moves = moves.enumerate().cycle();
        let mut rocks = 0;

        loop {
            let (piece_id, piece) = pieces.next().unwrap();
            self.piece_id = piece_id;

            let mut x = 2;
            let mut y = self.height + 4;

            loop {
                let (mov_id, mov) = moves.next().unwrap();
                self.mov_id = mov_id;

                let mov = mov as i64;

                if !self.overlap(piece, x + mov, y) {
                    x += mov;
                }

                if self.overlap(piece, x, y - 1) {
                    break;
                }
                y -= 1
            }

            self.rocks.extend(piece.iter().map(|&p| (p.0 + x, p.1 + y)));
            rocks += 1;
            self.height = self.rocks.iter().map(|p| p.1).max().unwrap();

            if callback(&mut self, rocks) {
                return self.height;
            }
        }
    }

    pub(crate) fn overlap(&self, piece: &Piece, x: i64, y: i64) -> bool {
        piece
            .iter()
            .any(|p| !(0..7).contains(&(x + p.0)) || self.rocks.contains(&(x + p.0, y + p.1)))
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

        let chamber = Chamber::new();
        let x = chamber.simulate(moves, |_, rocks| rocks == 2022);

        Ok(Box::new(x))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let moves = input.chars().map_while(|c| match c {
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => None,
        });

        let mut cache: HashMap<(usize, usize), (usize, i64)> = HashMap::new();

        let x = Chamber::new().simulate(moves, |chamber, rocks| {
            println!("{rocks}");
            let state = (chamber.piece_id, chamber.mov_id);
            let x = match cache.get(&state) {
                None => false,
                Some(&(last_rock_count, last_height)) => {
                    let add_rocks = rocks - last_rock_count;
                    let add_height = chamber.height - last_height;
                    if (1000000000000 - rocks) % add_rocks == 0 {
                        chamber.height +=
                            (1000000000000 - rocks as i64) / add_rocks as i64 * add_height;
                        true
                    } else {
                        false
                    }
                }
            };
            cache.insert(state, (rocks, chamber.height));
            x
        });

        Ok(Box::new(x))
    }
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_solution!(Day17.part_one, input, "3068");
    assert_solution!(Day17.part_two, input, "1514285714288");
}
