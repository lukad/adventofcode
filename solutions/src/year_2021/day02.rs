use aoc::*;

#[derive(Debug, Date)]
#[date(year = 2021, day = 2)]
pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> aoc::AocResult {
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
        Ok(Box::new(x * y))
    }

    fn part_two(&self, input: &str) -> aoc::AocResult {
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
        Ok(Box::new(x * y))
    }
}

#[test]
fn test() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
    assert_solution!(Day02.part_one, input, "150");
    assert_solution!(Day02.part_two, input, "900");
}
