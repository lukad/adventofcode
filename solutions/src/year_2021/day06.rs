use aoc::aoc;

fn step(fish: &mut Vec<i32>) {
    let mut new_fish = vec![];
    for n in fish.iter_mut() {
        match *n {
            0 => {
                *n = 6;
                new_fish.push(8);
            }
            _ => *n -= 1,
        }
    }
    fish.append(&mut new_fish);
}

#[aoc(year = 2021, day = 6, part = "one")]
fn solve_2021_06_01(input: &str) -> Box<i32> {
    let mut fish = input
        .trim()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    for _ in 0..80 {
        step(&mut fish);
    }

    Box::new(fish.len() as i32)
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "3,4,3,1,2";
    assert_eq!(
        solve_2021_06_01.solve(input).to_string(),
        "5934".to_string()
    );
}
