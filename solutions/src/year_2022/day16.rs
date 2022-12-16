use aoc::*;
use hashbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{self, complete::newline},
    combinator::{iterator, map},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Valve<'a> {
    id: u64,
    name: &'a str,
    rate: u64,
    tunnels: Vec<u64>,
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 16)]
pub struct Day16;

#[allow(clippy::too_many_arguments)]
fn max_flow(
    start: u64,
    current: u64,
    opened: u64,
    flows: &HashMap<u64, u64>,
    tunnels: &HashMap<u64, Vec<u64>>,
    mins_left: u8,
    elephants: u8,
    cache: &mut HashMap<(u64, u64, u8, u8), u64>,
) -> u64 {
    if mins_left == 0 {
        return if elephants > 0 {
            max_flow(
                start,
                start,
                opened,
                flows,
                tunnels,
                26,
                elephants - 1,
                cache,
            )
        } else {
            0
        };
    }

    let key = (current, opened, mins_left, elephants);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let mut pressure = 0;

    if opened & current == 0 {
        let val = (mins_left as u64 - 1) * flows.get(&current).unwrap();

        if val != 0 {
            let cur_opened = opened | current;
            for &neighbor in tunnels.get(&current).unwrap() {
                pressure = pressure.max(
                    val + max_flow(
                        start,
                        neighbor,
                        cur_opened,
                        flows,
                        tunnels,
                        mins_left - 2,
                        elephants,
                        cache,
                    ),
                );
            }
        }
    }

    for &neighbor in tunnels.get(&current).unwrap() {
        pressure = pressure.max(max_flow(
            start,
            neighbor,
            opened,
            flows,
            tunnels,
            mins_left - 1,
            elephants,
            cache,
        ));
    }

    cache.insert(key, pressure);
    pressure
}

// fn solve(input: &str, mins_left: u64);

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut tunnels = HashMap::new();
        let mut flows = HashMap::new();
        let mut ids = HashMap::new();

        let mut valves: Vec<_> = iterator(input, parse_line).into_iter().collect();

        for (i, valve) in valves.iter_mut().enumerate() {
            let new_id = 1 << i;
            ids.insert(valve.id, new_id);
            valve.id = new_id;
        }

        for valve in valves.iter_mut() {
            for tunnel in valve.tunnels.iter_mut() {
                *tunnel = *ids.get(tunnel).unwrap();
            }
        }

        for valve in valves.into_iter() {
            flows.insert(valve.id, valve.rate);
            tunnels.insert(valve.id, valve.tunnels);
        }

        let mut cache = HashMap::new();

        Ok(Box::new(max_flow(
            *ids.get(&370).unwrap(),
            *ids.get(&370).unwrap(),
            0,
            &flows,
            &tunnels,
            30,
            0,
            &mut cache,
        )))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut tunnels = HashMap::new();
        let mut flows = HashMap::new();
        let mut ids = HashMap::new();

        let mut valves: Vec<_> = iterator(input, parse_line).into_iter().collect();

        for (i, valve) in valves.iter_mut().enumerate() {
            let new_id = 1 << i;
            ids.insert(valve.id, new_id);
            valve.id = new_id;
        }

        dbg!(&ids);

        for valve in valves.iter_mut() {
            for tunnel in valve.tunnels.iter_mut() {
                *tunnel = *ids.get(tunnel).unwrap();
            }
        }

        for valve in valves.into_iter() {
            flows.insert(valve.id, valve.rate);
            tunnels.insert(valve.id, valve.tunnels);
        }

        let mut cache = HashMap::new();

        Ok(Box::new(max_flow(
            *ids.get(&370).unwrap(),
            *ids.get(&370).unwrap(),
            0,
            &flows,
            &tunnels,
            26,
            1,
            &mut cache,
        )))
    }
}

fn parse_line(i: &str) -> IResult<&str, Valve> {
    let (i, name) = preceded(tag("Valve "), take(2usize))(i)?;
    let id = u64::from_str_radix(name, 36).unwrap();
    let (i, rate) = preceded(tag(" has flow rate="), character::complete::u64)(i)?;
    let (i, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(i)?;
    let (i, tunnels) = separated_list1(
        tag(", "),
        map(take(2usize), |id| u64::from_str_radix(id, 36).unwrap()),
    )(i)?;
    let (i, _) = newline(i)?;
    Ok((
        i,
        Valve {
            id,
            name,
            rate,
            tunnels,
        },
    ))
}

#[test]
fn test() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
    assert_solution!(Day16.part_one, input, "1651");
    assert_solution!(Day16.part_two, input, "1707");
}