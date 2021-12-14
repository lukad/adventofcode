use aoc::aoc;

fn solve(input: &str, part_two: bool) -> isize {
    let crabs: Vec<isize> = input
        .trim()
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    let score = (*min..*max)
        .map(|point| {
            crabs.iter().fold(0, |acc, crab| {
                let d = (*crab - point).abs();
                acc + if part_two { d * (d + 1) / 2 } else { d }
            })
        })
        .min()
        .unwrap();

    score
}

#[aoc(year = 2021, day = 7, part = "one")]
fn solve_2021_07_01(input: &str) -> Box<isize> {
    Box::new(solve(input, false))
}

#[aoc(year = 2021, day = 7, part = "two")]
fn solve_2021_07_02(input: &str) -> Box<isize> {
    Box::new(solve(input, true))
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(solve_2021_07_01.solve(input).to_string(), "37".to_string());
    assert_eq!(solve_2021_07_02.solve(input).to_string(), "168".to_string());
}
