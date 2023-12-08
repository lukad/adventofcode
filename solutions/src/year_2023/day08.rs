use aoc::*;
use std::collections::BTreeMap;

use crate::util::LcmExt;

#[derive(Debug, Date)]
#[date(year = 2023, day = 8)]
pub struct Day08;

fn hash(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let tag = match bytes[2] {
        b'A' => 1,
        b'Z' => 2,
        _ => 0,
    };
    u32::from_be_bytes([bytes, &[tag]].concat().try_into().unwrap())
}

fn parse(s: &str) -> (Vec<char>, BTreeMap<u32, (u32, u32)>) {
    let (dirs, nodes) = s.split_once("\n\n").unwrap();
    let dirs = dirs.chars().collect();

    let map = nodes
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(" = ").unwrap();
            let values = values[1..values.len() - 1].split_once(", ").unwrap();
            (hash(key), (hash(values.0), hash(values.1)))
        })
        .collect();

    (dirs, map)
}

fn solve<I>(start: u32, nodes: &BTreeMap<u32, (u32, u32)>, dirs: I) -> u64
where
    I: Iterator<Item = char>,
{
    let mut i = 0;
    let mut node = nodes.get(&start).unwrap();
    for dir in dirs {
        i += 1;
        let next = match dir {
            'L' => node.0,
            _ => node.1,
        };
        if next & 2 != 0 {
            break;
        }
        node = nodes.get(&next).unwrap();
    }
    i
}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> AocResult {
        let (dirs, map) = parse(input);
        let steps = solve(hash("AAA"), &map, dirs.into_iter().cycle());
        Ok(Box::new(steps))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let (dirs, map) = parse(input);

        let steps = map
            .keys()
            .cloned()
            .filter(|key| *key & 0b0000_0001 != 0)
            .map(|key| solve(key, &map, dirs.clone().into_iter().cycle()))
            .collect::<Vec<_>>();

        let lcm: u64 = steps.into_iter().lcm();

        Ok(Box::new(lcm))
    }
}

#[test]
fn test() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let input2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_solution!(Day08.part_one, input, "2");
    assert_solution!(Day08.part_two, input2, "6");
}
