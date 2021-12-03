use std::env;
use std::io::{self, BufRead};

use itermore::IterMore;

fn solution<B: BufRead>(buf: B) -> i32 {
    let mut result = 0;
    let mut last = None;
    for line in buf.lines() {
        let num: i32 = line.unwrap().parse().unwrap();
        match last {
            None => (),
            Some(last_num) => {
                if num > last_num {
                    result += 1;
                }
            }
        }
        last = Some(num);
    }
    result
}

fn solution_part_two<B: BufRead>(buf: B) -> i32 {
    let mut result = 0;

    for [[a1, a2, a3], [b1, b2, b3]] in buf
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .windows()
        .windows()
    {
        if b1 + b2 + b3 > a1 + a2 + a3 {
            result += 1
        }
    }

    result
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
    let buf = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n".as_bytes();
    assert_eq!(solution(buf), 7);
    assert_eq!(solution_part_two(buf), 5);
}
