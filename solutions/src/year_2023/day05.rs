use std::ops::Range;

use itertools::Itertools;
use rayon::prelude::*;

use aoc::*;

struct Conversion {
    source: Range<isize>,
    offset: isize,
}

struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let conversions = input
            .lines()
            .skip(1)
            .map(|line| {
                let (a, b, c) = line
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap();

                Conversion {
                    source: b..b + c,
                    offset: b - a,
                }
            })
            .collect();
        Self { conversions }
    }

    fn convert(&self, seed: isize) -> isize {
        for conversion in &self.conversions {
            if conversion.source.contains(&seed) {
                return seed - conversion.offset;
            }
        }
        seed
    }
}

#[derive(Debug, Date)]
#[date(year = 2023, day = 5)]
pub struct Day05;

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> AocResult {
        let (seeds, _) = input.split_once('\n').unwrap();
        let seeds = seeds
            .split_ascii_whitespace()
            .skip(1)
            .map(|seed| seed.parse::<isize>().unwrap());
        let maps = input.split("\n\n").map(Map::parse).collect::<Vec<_>>();
        let min_location = seeds
            .into_iter()
            .map(|seed| maps.iter().fold(seed, |acc, map| map.convert(acc)))
            .min()
            .unwrap();
        Ok(Box::new(min_location))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (seeds, _) = input.split_once('\n').unwrap();
        let seeds = seeds
            .split_ascii_whitespace()
            .skip(1)
            .map(|seed| seed.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let seeds = seeds
            .chunks(2)
            .flat_map(|arr| arr[0]..arr[0] + arr[1])
            .collect::<Vec<_>>();
        let maps = input.split("\n\n").map(Map::parse).collect::<Vec<_>>();
        let min_location = seeds
            .into_par_iter()
            .map(|seed| maps.iter().fold(seed, |acc, map| map.convert(acc)))
            .min()
            .unwrap();
        Ok(Box::new(min_location))
    }
}

#[test]
fn test() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_solution!(Day05.part_one, input, "35");
    assert_solution!(Day05.part_two, input, "46");
}
