use std::fmt::Write;

use aoc::aoc;

struct Octopi {
    data: Vec<u8>,
}

impl std::fmt::Display for Octopi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self
            .data
            .iter()
            .map(|b| (b + b'0') as char)
            .collect::<Vec<char>>()
            .chunks(10)
        {
            for c in row {
                f.write_char(*c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Octopi {
    fn parse(input: &str) -> Self {
        let data = input
            .trim()
            .lines()
            .flat_map(|line| line.bytes().map(|c| (c - b'0')))
            .collect::<Vec<u8>>();
        Self { data }
    }

    fn step(&mut self) -> usize {
        for i in 0..self.data.len() {
            self.flash(i);
        }

        self.data
            .iter_mut()
            .filter(|cell| **cell > 9)
            .map(|cell| *cell = 0)
            .count()
    }

    fn flash(&mut self, i: usize) {
        self.data[i] += 1;

        let x = i % 10;
        let y = i / 10;

        let matches = (x > 0, x < 9, y > 0, y < 9);

        if self.data[i] == 10 {
            if matches.0 {
                self.flash(i - 1);
            }
            if matches.1 {
                self.flash(i + 1)
            }
            if matches.2 {
                self.flash(i - 10)
            }
            if matches.3 {
                self.flash(i + 10)
            }
            if matches.0 && matches.2 {
                self.flash(i - 10 - 1)
            }
            if matches.1 && matches.2 {
                self.flash(i - 10 + 1)
            }
            if matches.0 && matches.3 {
                self.flash(i + 10 - 1)
            }
            if matches.1 && matches.3 {
                self.flash(i + 10 + 1)
            }
        }
    }
}

#[aoc(year = 2021, day = 11, part = "one")]
pub fn solve_2021_11_01(input: &str) -> Box<usize> {
    let mut octopi = Octopi::parse(input);
    let result = (0..100).map(|_| octopi.step()).sum::<usize>();
    Box::new(result)
}

#[aoc(year = 2021, day = 11, part = "two")]
pub fn solve_2021_11_02(input: &str) -> Box<usize> {
    let mut octopi = Octopi::parse(input);
    let result = (1..).find(|_| octopi.step() == 100).unwrap();
    Box::new(result)
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    assert_eq!(
        solve_2021_11_01.solve(input).to_string(),
        "1656".to_string()
    );
    assert_eq!(solve_2021_11_02.solve(input).to_string(), "195".to_string());
}
