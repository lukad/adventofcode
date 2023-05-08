use aoc::*;
use hashbrown::HashMap;
// use fnv::FnvHashMap as HashMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{self, complete::newline},
    combinator::{iterator, map},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use pathfinding::prelude::dijkstra;

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

struct State {
    start: u64,
    flows: HashMap<u64, u64>,
    valves: Vec<u64>,
    distances: HashMap<(u64, u64), u8>,
    // initial_mins_left: u8,
}

impl State {
    fn solve(
        &self,
        current: u64,
        opened: u64,
        mins_left: u8,
        elephants: u8,
        cache: &mut HashMap<(u64, u64, u8, u8), u64>,
    ) -> u64 {
        if mins_left == 0 {
            return if elephants > 0 {
                self.solve(self.start, opened, 26, elephants - 1, cache)
            } else {
                0
            };
        }

        let key = (current, opened, mins_left, elephants);
        if let Some(value) = cache.get(&key) {
            return *value;
        }

        let mut pressure = 0;
        let mut max_pressure = 0;
        let mut opened = opened;
        let mut mins_left = mins_left;

        if current != self.start && opened & current == 0 {
            let &flow = self.flows.get(&current).unwrap();
            mins_left -= 1;
            pressure += (mins_left as u64) * flow;
            opened |= current;
        }

        for &neighbor in self.valves.iter() {
            if neighbor == current {
                continue;
            }
            let distance = *self.distances.get(&(current, neighbor)).unwrap();
            if distance > (mins_left) {
                continue;
            }
            let p =
                pressure + self.solve(neighbor, opened, mins_left - (distance), elephants, cache);
            max_pressure = max_pressure.max(p);
        }

        cache.insert(key, max_pressure);
        max_pressure
    }
}

fn solve(input: &str, initial_mins_left: u8, elephants: u8) -> u64 {
    let mut tunnels = HashMap::default();
    let mut flows = HashMap::default();
    let mut ids = HashMap::new();

    let mut valves: Vec<_> = iterator(input, parse_line).into_iter().collect();
    valves.sort_by_key(|v| v.name);

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

    for valve in valves.iter() {
        flows.insert(valve.id, valve.rate);
        tunnels.insert(valve.id, valve.tunnels.clone());
    }

    let mut flow_valves = valves
        .into_iter()
        .filter(|v| v.rate > 0 || v.id == 1)
        .map(|v| v.id)
        .collect::<Vec<_>>();

    let mut distances = HashMap::default();
    for i in 0..flow_valves.len() {
        for j in 0..flow_valves.len() {
            if i == j {
                continue;
            }
            let src = flow_valves[i];
            let dst = flow_valves[j];
            if distances.contains_key(&(src, dst)) || distances.contains_key(&(dst, src)) {
                continue;
            }
            let distance = path_length(src, dst, &tunnels);
            distances.insert((src, dst), distance);
            distances.insert((dst, src), distance);
        }
    }

    flow_valves.remove(0);

    let mut cache = HashMap::default();

    let state = State {
        start: *ids.get(&370).unwrap(),
        flows,
        valves: flow_valves,
        distances,
    };

    state.solve(state.start, 1, initial_mins_left, elephants, &mut cache)
}

fn path_length(src: u64, dst: u64, tunnels: &HashMap<u64, Vec<u64>>) -> u8 {
    let (_path, length) = dijkstra(
        &src,
        |cur| tunnels.get(cur).unwrap().iter().map(|&valve| (valve, 1)),
        |&cur| cur == dst,
    )
    .unwrap();
    length
}

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input, 30, 0)))
    }

    fn part_two(&self, input: &str) -> AocResult {
        Ok(Box::new(solve(input, 26, 1)))
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
