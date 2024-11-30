use std::collections::HashMap;

use aoc::*;
use itertools::Itertools;

#[derive(Debug, Date)]
#[date(year = 2023, day = 12)]
pub struct Day12;

fn solve(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    springs: &[u8],
    included: Option<usize>,
    remaining: &[usize],
) -> usize {
    if springs.is_empty() {
        return match (included, remaining.len()) {
            (None, 0) => 1,
            (Some(n), 1) if n == remaining[0] => 1,
            _ => 0,
        };
    }

    if included.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (springs.len(), included.unwrap_or(0), remaining.len());
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let arrangements = match (springs[0], included) {
        (b'.', Some(n)) if n != remaining[0] => 0,
        (b'.', Some(_)) => solve(cache, &springs[1..], None, &remaining[1..]),
        (b'.', None) => solve(cache, &springs[1..], None, remaining),
        (b'#', Some(n)) => solve(cache, &springs[1..], Some(n + 1), remaining),
        (b'#', None) => solve(cache, &springs[1..], Some(1), remaining),
        (b'?', Some(n)) => {
            let mut res = solve(cache, &springs[1..], Some(n + 1), remaining);
            if n == remaining[0] {
                res += solve(cache, &springs[1..], None, &remaining[1..]);
            }
            res
        }
        (b'?', None) => {
            solve(cache, &springs[1..], Some(1), remaining)
                + solve(cache, &springs[1..], None, remaining)
        }
        _ => unreachable!(),
    };

    cache.insert(key, arrangements);

    arrangements
}

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> AocResult {
        let mut sum = 0;
        for line in input.lines() {
            let (springs, groups) = line.split_once(' ').unwrap();
            let groups = groups.split(',').map(|x| x.parse().unwrap()).collect_vec();
            sum += solve(&mut HashMap::new(), springs.as_bytes(), None, &groups);
        }

        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut sum = 0;
        for line in input.lines() {
            let (springs, groups) = line.split_once(' ').unwrap();
            let springs = (0..5).map(|_| springs).join("?");
            let groups = groups.split(',').map(|x| x.parse().unwrap()).collect_vec();
            let groups = (0..5).flat_map(|_| &groups).copied().collect::<Vec<_>>();
            sum += solve(&mut HashMap::new(), springs.as_bytes(), None, &groups);
        }

        Ok(Box::new(sum))
    }
}
#[test]
fn test() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_solution!(Day12.part_one, input, "21");
    assert_solution!(Day12.part_two, input, "525152");
}
