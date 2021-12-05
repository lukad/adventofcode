use std::env;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Cell {
    Unmarked(i32),
    Marked(i32),
}

impl Cell {
    fn value(self) -> i32 {
        match self {
            Cell::Unmarked(x) => x,
            Cell::Marked(x) => x,
        }
    }
}

impl Cell {
    fn marked(&self) -> bool {
        match self {
            Cell::Unmarked(_) => false,
            Cell::Marked(_) => true,
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: [[Cell; 5]; 5],
    won: bool,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let mut cells = [[Cell::Unmarked(0); 5]; 5];
        for (i, num) in value.split_ascii_whitespace().take(25).enumerate() {
            let x = i % 5;
            let y = i / 5;
            cells[y][x] = Cell::Unmarked(num.parse().unwrap());
        }
        Board { cells, won: false }
    }
}

impl Board {
    fn mark(&mut self, value: i32) -> bool {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value() == value {
                    *cell = Cell::Marked(value)
                }
            }
        }
        let won = self.has_won();
        let changed_to_won = !self.won && self.has_won();
        self.won = won;
        changed_to_won
    }

    fn has_won(&self) -> bool {
        if self
            .cells
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked()))
        {
            return true;
        }

        for row in 0..5 {
            let mut marked = 0;
            for col in 0..5 {
                if self.cells[col][row].marked() {
                    marked += 1;
                }
                if marked == 5 {
                    return true;
                }
            }
        }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        self.cells
            .iter()
            .map(|row| {
                row.iter().fold(0, |sum, cell| match cell {
                    &Cell::Unmarked(x) => sum + x,
                    Cell::Marked(_) => sum,
                })
            })
            .sum()
    }
}

fn parse_game<B: BufRead>(mut buf: B) -> (Vec<i32>, Vec<Board>) {
    let mut s = "".to_string();
    buf.read_to_string(&mut s).unwrap();

    let (nums, boards) = s.split_once("\n\n").unwrap();

    let nums: Vec<i32> = nums.split(",").map(|s| s.parse().unwrap()).collect();
    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|board_str| board_str.into())
        .collect();

    (nums, boards)
}

fn solution<B: BufRead>(buf: B) -> i32 {
    let (nums, mut boards) = parse_game(buf);

    for num in nums.iter() {
        for board in boards.iter_mut() {
            if board.mark(*num) {
                return board.unmarked_sum() * num;
            }
        }
    }

    0
}

fn solution_part_two<B: BufRead>(buf: B) -> i32 {
    let (nums, mut boards) = parse_game(buf);

    let mut last_winner_score = 0;

    for num in nums.iter() {
        for board in boards.iter_mut() {
            if board.mark(*num) {
                last_winner_score = board.unmarked_sum() * num;
            }
        }
    }

    last_winner_score
}

fn main() {
    let stdin = io::stdin();
    match env::args().nth(1).as_ref().map(|x| x.as_str()) {
        None => println!("{:?}", solution(stdin.lock())),
        Some("--part-two") => println!("{:?}", solution_part_two(stdin.lock())),
        _ => println!("uknown options"),
    }
}

#[test]
fn test_01() {
    let buf = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        .as_bytes();
    assert_eq!(solution(buf), 4512);
    assert_eq!(solution_part_two(buf), 1924);
}
