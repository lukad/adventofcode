use aoc::*;

#[derive(Debug, Default)]
struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn parse(s: &str) -> Self {
        let (id, rest) = s.split_once(": ").unwrap();

        let mut game = Self {
            id: id.split_once(' ').unwrap().1.parse().unwrap(),
            ..Default::default()
        };

        for set in rest.trim_start().split("; ") {
            for cubes in set.split(", ") {
                let (amount, color) = cubes.split_once(' ').unwrap();
                let amount = amount.parse::<usize>().unwrap();
                match color {
                    "red" => game.red = amount.max(game.red),
                    "green" => game.green = amount.max(game.green),
                    "blue" => game.blue = amount.max(game.blue),
                    _ => (),
                }
            }
        }

        game
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, Date)]
#[date(year = 2023, day = 2)]
pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut sum = 0;

        for line in input.lines() {
            let game = Game::parse(line);
            if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
                sum += game.id;
            }
        }

        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut sum = 0;

        for line in input.lines() {
            sum += Game::parse(line).power();
        }

        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_solution!(Day02.part_one, input, "8");
    assert_solution!(Day02.part_two, input, "2286");
}
