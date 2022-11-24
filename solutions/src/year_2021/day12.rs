use std::collections::{HashMap, HashSet};

use aoc::aoc;

fn is_small(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

struct System<'a> {
    caves: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> From<&'a str> for System<'a> {
    fn from(s: &'a str) -> Self {
        let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

        for line in s.trim().lines() {
            let (from, to) = line.split_once('-').unwrap();
            caves.entry(from).or_default().push(to);
            caves.entry(to).or_default().push(from);
        }

        Self { caves }
    }
}

impl<'a> System<'a> {
    fn find_paths(&self, visit_small_caves_twice: bool) -> usize {
        let mut visited = HashSet::new();
        visited.insert("start");
        self.do_find_paths("start", visited, visit_small_caves_twice)
    }

    fn do_find_paths(
        &self,
        from: &'a str,
        visited: HashSet<&'a str>,
        visit_small_caves_twice: bool,
    ) -> usize {
        if from == "end" {
            return 1;
        }

        let mut count = 0;

        for to in self.caves.get(from).unwrap() {
            let mut visited = visited.clone();
            let mut visit_small_caves_twice = visit_small_caves_twice;
            if is_small(to) {
                if visited.contains(to) {
                    if visit_small_caves_twice && *to != "start" {
                        visit_small_caves_twice = false;
                    } else {
                        continue;
                    }
                } else {
                    visited.insert(*to);
                }
            }

            count += self.do_find_paths(to, visited, visit_small_caves_twice);
        }

        count
    }
}

#[aoc(year = 2021, day = 12, part = "one")]
fn solve_2021_12_01(input: &str) -> Box<i32> {
    let system = System::from(input);
    Box::new(system.find_paths(false))
}

#[aoc(year = 2021, day = 12, part = "two")]
fn solve_2021_12_02(input: &str) -> Box<i32> {
    let system = System::from(input);
    Box::new(system.find_paths(true))
}

#[test]
fn test() {
    use aoc::Solution;
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    assert_eq!(solve_2021_12_01.solve(input).to_string(), "19".to_string());
    assert_eq!(solve_2021_12_02.solve(input).to_string(), "103".to_string());
}
