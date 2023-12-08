use std::cmp::Reverse;

use aoc::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    bid: u32,
    strength: Vec<u32>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (a, b) in self.strength.iter().zip(other.strength.iter()) {
            match a.cmp(b) {
                std::cmp::Ordering::Equal => {}
                other => return other,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn parse(s: &str, jokers: bool) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse::<u32>().unwrap();
        let cards = cards
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' if jokers => 0,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                c if c.is_ascii_digit() => c.to_digit(10).unwrap(),
                _ => panic!("Invalid card"),
            })
            .collect::<Vec<_>>();

        let mut freq = cards.iter().counts();

        let joker_count = if jokers {
            let count = freq.get(&0).copied().unwrap_or(0);
            freq.remove(&0);
            count
        } else {
            0
        };

        let strength = freq
            .into_values()
            .map(|v| v as u32)
            .sorted_by_key(|&v| Reverse(v))
            .collect();

        let mut strength = [strength, cards].concat();
        if jokers {
            strength[0] += joker_count as u32;
        }
        Self { strength, bid }
    }
}

#[derive(Debug, Date)]
#[date(year = 2023, day = 7)]
pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> AocResult {
        let sum: u32 = input
            .lines()
            .map(|l| Hand::parse(l, false))
            .sorted()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
            .sum();
        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let sum: u32 = input
            .lines()
            .map(|l| Hand::parse(l, true))
            .sorted()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
            .sum();
        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_solution!(Day07.part_one, input, "6440");
    assert_solution!(Day07.part_two, input, "5905");
}
