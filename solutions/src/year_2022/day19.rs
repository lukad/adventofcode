use aoc::*;
use hashbrown::HashMap;
use rayon::prelude::*;

#[derive(Debug)]
struct Blueprint {
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: (u8, u8),
    geode_robot_cost: (u8, u8),
    max_ore_cost: u8,
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let nums = s
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect::<Vec<_>>();

        let ore = nums[0];
        let clay = nums[1];
        let obsidian = (nums[2], nums[3]);
        let geode = (nums[4], nums[5]);

        Blueprint {
            ore_cost: ore,
            clay_cost: clay,
            obsidian_cost: obsidian,
            geode_robot_cost: geode,
            max_ore_cost: [ore, clay, obsidian.0, obsidian.0]
                .into_iter()
                .max()
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct State {
    ore_bots: u8,
    clay_bots: u8,
    obsidian_bots: u8,

    ore_count: u8,
    clay_count: u8,
    obsidian_count: u8,

    time_left: u8,
}

impl State {
    fn new(time_left: u8) -> Self {
        State {
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,

            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,

            time_left,
        }
    }

    fn simulate(&mut self, minutes: u8) {
        self.ore_count += self.ore_bots * minutes;
        self.clay_count += self.clay_bots * minutes;
        self.obsidian_count += self.obsidian_bots * minutes;
        self.time_left -= minutes;
    }

    fn build_ore_bot(&mut self, bp: &Blueprint) {
        self.ore_count -= bp.ore_cost;
        self.ore_bots += 1;
    }

    fn build_clay_bot(&mut self, bp: &Blueprint) {
        self.ore_count -= bp.clay_cost;
        self.clay_bots += 1;
    }

    fn build_obsidian_bot(&mut self, bp: &Blueprint) {
        self.ore_count -= bp.obsidian_cost.0;
        self.clay_count -= bp.obsidian_cost.1;
        self.obsidian_bots += 1;
    }

    fn build_geode_bot(&mut self, bp: &Blueprint) {
        self.ore_count -= bp.geode_robot_cost.0;
        self.obsidian_count -= bp.geode_robot_cost.1;
    }

    fn time_to_build_ore_bot(&self, bp: &Blueprint) -> u8 {
        (bp.ore_cost.saturating_sub(self.ore_count)).div_ceil(self.ore_bots) + 1
    }

    fn time_to_build_clay_bot(&self, bp: &Blueprint) -> u8 {
        (bp.clay_cost.saturating_sub(self.ore_count)).div_ceil(self.ore_bots) + 1
    }

    fn time_to_build_obsidian_bot(&self, bp: &Blueprint) -> Option<u8> {
        if self.clay_bots == 0 {
            None
        } else {
            let ore_mins =
                (bp.obsidian_cost.0.saturating_sub(self.ore_count)).div_ceil(self.ore_bots) + 1;
            let clay_mins =
                (bp.obsidian_cost.1.saturating_sub(self.clay_count)).div_ceil(self.clay_bots) + 1;
            Some(ore_mins.max(clay_mins))
        }
    }

    fn time_to_build_geode_bot(&self, bp: &Blueprint) -> Option<u8> {
        if self.obsidian_bots == 0 {
            None
        } else {
            let ore_mins =
                (bp.geode_robot_cost.0.saturating_sub(self.ore_count)).div_ceil(self.ore_bots) + 1;
            let obsidian_mins = (bp.geode_robot_cost.1.saturating_sub(self.obsidian_count))
                .div_ceil(self.obsidian_bots)
                + 1;
            Some(ore_mins.max(obsidian_mins))
        }
    }
}

type Cache = HashMap<State, u8>;

fn dfs(state: &State, bp: &Blueprint, current: u8, max: &mut u8, cache: &mut Cache) -> u8 {
    if state.time_left == 0 {
        return current;
    }

    if let Some(&cached) = cache.get(state) {
        return cached;
    }

    let n: u16 = if state.obsidian_count >= bp.geode_robot_cost.1 {
        state.time_left as u16
    } else {
        (state.time_left as u16).saturating_sub(1).max(1)
    };

    let n = n * (n - 1) / 2;

    if current as u16 + n <= *max as u16 {
        return 0;
    }

    let mut best = 0;

    if let Some(minutes) = state.time_to_build_geode_bot(bp) {
        if minutes < state.time_left {
            let mut next_state = state.clone();
            next_state.simulate(minutes);
            next_state.build_geode_bot(bp);
            let new_score = next_state.time_left;
            best = dfs(&mut next_state, bp, current + new_score, &mut best, cache) + new_score;
        }
    }

    if bp.geode_robot_cost.1 > state.obsidian_bots && state.time_left >= 3 {
        if let Some(minutes) = state.time_to_build_obsidian_bot(bp) {
            if minutes < state.time_left {
                let mut next_state = state.clone();
                next_state.simulate(minutes);
                next_state.build_obsidian_bot(bp);
                best = best.max(dfs(&mut next_state, bp, current, &mut best, cache));
            }
        }
    }

    if bp.obsidian_cost.1 > state.clay_bots && state.time_left >= 4 {
        let minutes = state.time_to_build_clay_bot(bp);
        if minutes < state.time_left {
            let mut next_state = state.clone();
            next_state.simulate(minutes);
            next_state.build_clay_bot(bp);
            best = best.max(dfs(&mut next_state, bp, current, &mut best, cache));
        }
    }

    if bp.max_ore_cost > state.ore_bots && state.time_left >= 3 {
        let minutes = state.time_to_build_ore_bot(bp);
        if minutes < state.time_left {
            let mut next_state = state.clone();
            next_state.simulate(minutes);
            next_state.build_ore_bot(bp);
            best = best.max(dfs(&mut next_state, bp, current, &mut best, cache));
        }
    }

    cache.insert(state.clone(), best);

    *max = (*max).max(best + current);

    best
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 19)]
pub struct Day19;

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> AocResult {
        let lines = input.lines().collect::<Vec<&str>>();
        let result: u64 = lines
            .par_iter()
            .enumerate()
            .map(|(i, line)| {
                let bp = Blueprint::parse(line);
                let mut state = State::new(24);
                let mut max = 0;
                let score = dfs(&mut state, &bp, 0, &mut max, &mut HashMap::new()) as u64;
                score * (i + 1) as u64
            })
            .sum();
        Ok(Box::new(result))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let lines = input.lines().collect::<Vec<&str>>();
        let result: u64 = lines
            .par_iter()
            .take(3)
            .map(|line| {
                let bp = Blueprint::parse(line);
                let mut state = State::new(32);
                let mut max = 0;
                dfs(&mut state, &bp, 0, &mut max, &mut HashMap::new()) as u64
            })
            .product();
        Ok(Box::new(result))
    }
}

#[test]
fn test() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    assert_solution!(Day19.part_one, input, "33");
}

#[test]
#[cfg_attr(not(feature = "slow-tests"), ignore)]
fn slow_tests() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    assert_solution!(Day19.part_two, input, "3472"); //too slow in debug mode
}
