use itermore::IterMore;

use aoc::aoc;

#[aoc(year = 2021, day = 1, part = "one")]
pub fn solve_2021_01_01(input: &str) -> Box<i32> {
    let mut result = 0;
    let mut last = None;
    for line in input.lines() {
        let num: i32 = line.parse().unwrap();
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
    Box::new(result)
}

#[aoc(year = 2021, day = 1, part = "two")]
fn solve_2021_01_02(input: &str) -> Box<i32> {
    let mut result = 0;

    for [[a1, a2, a3], [b1, b2, b3]] in input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .windows()
        .windows()
    {
        if b1 + b2 + b3 > a1 + a2 + a3 {
            result += 1
        }
    }

    Box::new(result)
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    assert_eq!(solve_2021_01_01.solve(input).to_string(), "7".to_string());
    assert_eq!(solve_2021_01_02.solve(input).to_string(), "5".to_string());
}
