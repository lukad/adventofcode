use std::cmp::Ordering;

use aoc::*;
use nom::{
    branch::alt, character::complete::char, combinator::map, multi::separated_list0,
    sequence::delimited, IResult,
};

#[derive(Debug, Date)]
#[date(year = 2022, day = 13)]
pub struct Day13;

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> AocResult {
        let packets = parse_packets(input).collect::<Vec<_>>();

        let in_right_order: usize = packets
            .chunks_exact(2)
            .enumerate()
            .filter(|(_, packets)| packets[0] < packets[1])
            .map(|(i, _)| i + 1)
            .sum();

        Ok(Box::new(in_right_order))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let a = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let b = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

        let dividers = [a.clone(), b.clone()];

        let mut packets = parse_packets(input).chain(dividers).collect::<Vec<_>>();

        packets.sort();

        let pos1 = packets.iter().position(|p| p == &a).unwrap_or_default() + 1;
        let pos2 = packets.iter().position(|p| p == &b).unwrap_or_default() + 1;

        Ok(Box::new(pos1 * pos2))
    }
}

#[derive(Debug, Clone, Eq)]
enum Packet {
    Int(u8),
    List(Vec<Self>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::List(lhs), Self::List(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(lhs), Self::Int(rhs)) => lhs.partial_cmp(rhs),
            (Self::Int(_), Self::List(_)) => Self::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Int(_)) => self.partial_cmp(&Self::List(vec![other.clone()])),
            (Self::List(lhs), Self::List(rhs)) => lhs
                .iter()
                .zip(rhs)
                .map(|(l, r)| l.partial_cmp(r))
                .find(|o| o != &Some(Ordering::Equal))
                .unwrap_or_else(|| lhs.len().partial_cmp(&rhs.len())),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_packets(input: &'_ str) -> impl Iterator<Item = Packet> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|packet| parse_packet(packet).ok())
        .map(|(_, packet)| packet)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(nom::character::complete::u8, Packet::Int),
        map(
            delimited(
                char('['),
                separated_list0(char(','), parse_packet),
                char(']'),
            ),
            Packet::List,
        ),
    ))(input)
}

#[test]
fn test() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_solution!(Day13.part_one, input, "13");
    assert_solution!(Day13.part_two, input, "140");
}
