use aoc::*;
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::many1,
    Finish, IResult,
};

#[derive(Debug, Date)]
#[date(year = 2022, day = 22)]
pub struct Day22;

#[derive(Debug, Clone)]
enum Move {
    Left,
    Right,
    Forward(i32),
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        value(Move::Left, tag("L")),
        value(Move::Right, tag("R")),
        map(nom::character::complete::i32, Move::Forward),
    )))(input)
}

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> AocResult {
        let (map, moves, _face_size) = parse_input(input);
        let password = walk(&map, &moves, wrap);
        Ok(Box::new(password))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (map, moves, face_size) = parse_input(input);
        let password = walk(&map, &moves, |map, pos, dir| {
            wrap_cube(map, pos, dir, face_size)
        });
        Ok(Box::new(password))
    }
}

fn wrap_cube(_map: &[Vec<char>], pos: IVec2, dir: IVec2, face_size: usize) -> (IVec2, IVec2) {
    let face_size = face_size as i32;
    let old_local = pos % face_size;
    let (new_face, new_dir) = match (pos.x / face_size, pos.y / face_size, dir) {
        (2, 1, IVec2::X) => (IVec2::new(3, 2), IVec2::Y),
        (2, 2, IVec2::Y) => (IVec2::new(0, 1), IVec2::NEG_Y),
        (1, 1, IVec2::NEG_Y) => (IVec2::new(2, 0), IVec2::X),
        (1, 0, IVec2::NEG_Y) => (IVec2::new(0, 3), IVec2::X),
        (0, 3, IVec2::NEG_X) => (IVec2::new(1, 0), IVec2::Y),
        (2, 0, IVec2::Y) => (IVec2::new(1, 1), IVec2::NEG_X),
        (0, 2, IVec2::NEG_Y) => (IVec2::new(1, 1), IVec2::X),
        (1, 1, IVec2::NEG_X) => (IVec2::new(0, 2), IVec2::Y),
        (2, 0, IVec2::X) => (IVec2::new(1, 2), IVec2::NEG_X),
        (1, 2, IVec2::Y) => (IVec2::new(0, 3), IVec2::NEG_X),
        (0, 3, IVec2::X) => (IVec2::new(1, 2), IVec2::NEG_Y),
        (1, 2, IVec2::X) => (IVec2::new(2, 0), IVec2::NEG_X),
        (0, 2, IVec2::NEG_X) => (IVec2::new(1, 0), IVec2::X),
        (1, 0, IVec2::NEG_X) => (IVec2::new(0, 2), IVec2::X),
        (1, 1, IVec2::X) => (IVec2::new(2, 0), IVec2::NEG_Y),
        (2, 0, IVec2::NEG_Y) => (IVec2::new(0, 3), IVec2::NEG_Y),
        (0, 3, IVec2::Y) => (IVec2::new(2, 0), IVec2::Y),
        (x, y, d) => todo!("{x:?} {y:?} {d:?}"),
    };
    let local = match (dir, new_dir) {
        (IVec2::NEG_Y, IVec2::X) => IVec2::new(0, old_local.x),
        (IVec2::NEG_X, IVec2::Y) => IVec2::new(old_local.y, 0),
        (IVec2::NEG_X, IVec2::X) => IVec2::new(0, face_size - 1 - old_local.y),
        (IVec2::X, IVec2::NEG_Y) => IVec2::new(old_local.y, face_size - 1),
        (IVec2::X, IVec2::Y) => IVec2::new(face_size - 1 - old_local.y, 0),
        (IVec2::Y, IVec2::NEG_Y) => IVec2::new(face_size - 1 - old_local.x, face_size - 1),
        (IVec2::Y, IVec2::NEG_X) => IVec2::new(face_size - 1, old_local.x),
        (IVec2::X, IVec2::NEG_X) => IVec2::new(face_size - 1, face_size - 1 - old_local.y),
        (IVec2::NEG_Y, IVec2::NEG_Y) => IVec2::new(old_local.x, face_size - 1),
        (IVec2::Y, IVec2::Y) => IVec2::new(old_local.x, 0),
        (a, b) => todo!("foo {a:?} {b:?}"),
    };
    (new_face * face_size + local, new_dir)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>, usize) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let max_w = grid.lines().map(|line| line.len()).max().unwrap();
    let max_h = grid.lines().count();
    let face_size = max_w.min(max_h) / 3;
    let map = grid
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let (_, moves) = parse_moves(moves).finish().unwrap();
    (map, moves, face_size)
}

fn walk(
    map: &[Vec<char>],
    moves: &[Move],
    wrap: impl Fn(&[Vec<char>], IVec2, IVec2) -> (IVec2, IVec2),
) -> i32 {
    let mut pos = IVec2::ZERO;
    let mut dir = IVec2::X;

    while map[0][pos.x as usize] != '.' {
        pos.x += 1;
    }

    for mov in moves {
        match mov {
            Move::Left => dir = IVec2::new(dir.y, -dir.x),
            Move::Right => dir = IVec2::new(-dir.y, dir.x),
            Move::Forward(n) => {
                for _ in 0..*n {
                    let next = pos + dir;
                    match map
                        .get(next.y as usize)
                        .and_then(|row| row.get(next.x as usize))
                        .unwrap_or(&' ')
                    {
                        '.' => pos = next,
                        '#' => break,
                        ' ' => {
                            let (wrapped_next, wrapped_dir) = wrap(map, pos, dir);
                            if map[wrapped_next.y as usize][wrapped_next.x as usize] == '#' {
                                break;
                            }
                            pos = wrapped_next;
                            dir = wrapped_dir;
                        }
                        c => panic!("illegal map character: {c:?}"),
                    }
                }
            }
        }
    }

    let d = match dir {
        IVec2::X => 0,
        IVec2::Y => 1,
        IVec2::NEG_X => 2,
        IVec2::NEG_Y => 3,
        _ => unreachable!(),
    };

    (pos.y + 1) * 1000 + (pos.x + 1) * 4 + d
}

fn wrap(map: &[Vec<char>], pos: IVec2, dir: IVec2) -> (IVec2, IVec2) {
    let mut pos = pos;
    while map
        .get((pos.y - dir.y) as usize)
        .and_then(|row| row.get((pos.x - dir.x) as usize))
        .unwrap_or(&' ')
        != &' '
    {
        pos -= dir;
    }
    (pos, dir)
}

#[test]
fn test() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    assert_solution!(Day22.part_one, input, "6032");
    assert_solution!(Day22.part_two, input, "5031");
}
