use aoc::aoc;

fn solve(input: &str, window_size: usize) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<_>().unwrap())
        .collect::<Vec<usize>>()
        .windows(window_size)
        .fold(0, |acc, window| {
            if window.last().unwrap() > window.first().unwrap() {
                acc + 1
            } else {
                acc
            }
        })
}

#[aoc(year = 2021, day = 1, part = "one")]
pub fn solve_2021_01_01(input: &str) -> Box<i32> {
    Box::new(solve(input, 2))
}

#[aoc(year = 2021, day = 1, part = "two")]
fn solve_2021_01_02(input: &str) -> Box<i32> {
    Box::new(solve(input, 3))
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    assert_eq!(solve_2021_01_01.solve(input).to_string(), "7".to_string());
    assert_eq!(solve_2021_01_02.solve(input).to_string(), "5".to_string());
}
