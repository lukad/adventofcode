use aoc::*;
use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug, Date)]
#[date(year = 2022, day = 3)]
pub struct Day03;

fn to_priority(c: char) -> u64 {
    if c.is_lowercase() {
        return c as u64 - 96;
    }
    (c as u64 - 64) + 26
}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut sum = 0;
        for line in input.lines() {
            let line = line
                .chars()
                .into_iter()
                .map(to_priority)
                .collect::<Vec<_>>();
            let (l, r) = line.split_at(line.len() / 2);
            let mut hr: HashSet<u64> = HashSet::new();
            let mut hl: HashSet<u64> = HashSet::new();
            hl.extend(l);
            hr.extend(r);
            sum += hl.intersection(&hr).sum::<u64>();
        }
        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut sum = 0;
        for foos in input
            .lines()
            .map(|line| HashSet::<u64>::from_iter(line.chars().map(to_priority)))
            .chunks(3)
            .into_iter()
        {
            let sets = foos.collect::<Vec<_>>();
            let set1 = &sets[0];
            let other_sets = &sets[1..];
            sum += set1
                .iter()
                .filter(|k| other_sets.iter().all(|set| set.contains(*k)))
                .sum::<u64>();
        }
        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    assert_solution!(Day03.part_one, input, "157");
    assert_solution!(Day03.part_two, input, "70");
}
