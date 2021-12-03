use std::env;
use std::io::{self, BufRead};

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn solution<B: BufRead>(buf: B) -> i32 {
    let nums: Vec<Vec<usize>> = buf
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let row_count = nums.len();
    let row_size = nums[0].len();

    let nums = transpose(nums);
    let nums: Vec<usize> = nums
        .into_iter()
        .map(|col| col.into_iter().reduce(|acc, x| acc + x).unwrap())
        .map(|x| if x > row_count / 2 { 1 } else { 0 })
        .collect();

    let gamma = nums
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (bit, value)| acc | (value << bit));

    let mask = (1 << row_size) - 1;
    let epsilon = !gamma & mask;
    (gamma * epsilon) as i32
}

fn solution_part_two<B: BufRead>(_buf: B) -> i32 {
    0
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
    let buf =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"
            .as_bytes();
    assert_eq!(solution(buf), 198);
}
