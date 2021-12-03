use std::env;
use std::io::{self, BufRead};

fn solution<B: BufRead>(buf: B) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in buf.lines() {
        let line = line.unwrap();
        let (dir, length) = &line.split_once(' ').unwrap();
        let length: i32 = length.parse().unwrap();
        match dir {
            &"up" => y -= length,
            &"down" => y += length,
            &"forward" => x += length,
            _ => panic!("Unknown direction"),
        }
    }
    x * y
}

fn solution_part_two<B: BufRead>(buf: B) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    for line in buf.lines() {
        let line = line.unwrap();
        let (dir, length) = &line.split_once(' ').unwrap();
        let length: i32 = length.parse().unwrap();
        match dir {
            &"up" => aim -= length,
            &"down" => aim += length,
            &"forward" => {
                x += length;
                y += length * aim;
            }
            _ => panic!("Unknown direction"),
        }
    }
    x * y
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
    let buf = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n".as_bytes();

    assert_eq!(solution(buf), 150);
    assert_eq!(solution_part_two(buf), 900);
}
