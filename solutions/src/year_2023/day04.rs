use std::collections::HashSet;

use aoc::*;

#[derive(Debug, Clone)]
struct Card {
    winning: HashSet<usize>,
    have: HashSet<usize>,
}

impl Card {
    fn parse(s: &str) -> Self {
        let (_id, rest) = s.split_once(": ").unwrap();

        let (winning, have) = rest.split_once(" | ").unwrap();
        let winning = winning
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let have = have
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<HashSet<_>>();

        Self { winning, have }
    }
}

#[derive(Debug, Date)]
#[date(year = 2023, day = 4)]
pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> AocResult {
        let cards = input.lines().map(Card::parse).collect::<Vec<_>>();
        let sum: usize = cards
            .into_iter()
            .map(|card| card.winning.intersection(&card.have).count())
            .filter(|&n| n > 0)
            .map(|n| 2u32.pow(n as u32 - 1) as usize)
            .sum();
        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut cards = input
            .lines()
            .map(Card::parse)
            .map(|c| (c, 1))
            .collect::<Vec<_>>();

        for i in 0..cards.len() {
            let (_card, count) = cards[i].clone();
            let wins = cards[i].0.winning.intersection(&cards[i].0.have).count();
            if wins < 1 {
                continue;
            }
            for (_other, c) in cards[(i + 1)..].iter_mut().take(wins) {
                *c += count;
            }

            dbg!(cards.iter().map(|(_, c)| c).collect::<Vec<_>>());
        }

        let sum: usize = cards.into_iter().map(|(_card, count)| count).sum();

        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_solution!(Day01.part_one, input1, "13");
    assert_solution!(Day01.part_two, input1, "30");
}
