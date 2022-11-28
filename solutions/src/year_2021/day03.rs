use aoc::*;

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|el| el[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Debug, Date)]
#[date(year = 2021, day = 3)]
pub struct Day03;

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> aoc::AocResult {
        let nums: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
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
            .map(|x| usize::from(x > row_count / 2))
            .collect();

        let gamma = nums
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (bit, value)| acc | (value << bit));

        let mask = (1 << row_size) - 1;
        let epsilon = !gamma & mask;
        Ok(Box::new(gamma * epsilon))
    }
}

#[test]
fn test() {
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
    assert_solution!(Day03.part_one, input, "198");
}
