use aoc::aoc;

#[aoc(year = 2021, day = 2, part = "one")]
fn solve_2021_02_01(input: &str) -> Box<i32> {
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        let (dir, length) = line.split_once(' ').unwrap();
        let length: i32 = length.parse().unwrap();
        match dir {
            "up" => y -= length,
            "down" => y += length,
            "forward" => x += length,
            _ => panic!("Unknown direction"),
        }
    }
    Box::new(x * y)
}

#[aoc(year = 2021, day = 2, part = "two")]
fn solve_2021_02_02(input: &str) -> Box<i32> {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    for line in input.lines() {
        let (dir, length) = line.split_once(' ').unwrap();
        let length: i32 = length.parse().unwrap();
        match dir {
            "up" => aim -= length,
            "down" => aim += length,
            "forward" => {
                x += length;
                y += length * aim;
            }
            _ => panic!("Unknown direction"),
        }
    }
    Box::new(x * y)
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
    assert_eq!(solve_2021_02_01.solve(input).to_string(), "150".to_string());
    assert_eq!(solve_2021_02_02.solve(input).to_string(), "900".to_string());
}
