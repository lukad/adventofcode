use aoc::*;

#[derive(Debug, Date)]
#[date(year = 2023, day = 9)]
pub struct Day09;

struct History {
    history: Vec<i64>,
}

impl History {
    fn extrapolate_forward(&self) -> i64 {
        let mut changes = self.changes();
        let last_index = changes.len() - 1;
        changes[last_index].push(0);
        for i in (0..changes.len() - 2).rev() {
            let diff = changes[i].last().unwrap() + changes[i + 1].last().unwrap();
            changes[i].push(diff);
        }
        *changes[0].last().unwrap()
    }

    fn extrapolate_backward(&self) -> i64 {
        let mut changes = self.changes();
        let last_index = changes.len() - 1;
        changes[last_index].insert(0, 0);
        for i in (0..changes.len() - 2).rev() {
            let diff = changes[i][0] - changes[i + 1][0];
            changes[i].insert(0, diff);
        }
        *changes[0].first().unwrap()
    }

    fn changes(&self) -> Vec<Vec<i64>> {
        let mut changes = vec![self.history.clone()];
        let mut i = 0;
        while !changes[i].iter().all(|x| *x == 0) {
            let mut next = vec![];
            for items in changes[i].windows(2) {
                next.push(items[1] - items[0]);
            }
            changes.push(next);
            i += 1;
        }
        changes
    }
}

fn parse(input: &str) -> impl Iterator<Item = History> + '_ {
    input.lines().map(|line| {
        let values = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        History { history: values }
    })
}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> AocResult {
        let histories = parse(input);
        let sum = histories
            .map(|history| history.extrapolate_forward())
            .sum::<i64>();
        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let histories = parse(input);
        let sum = histories
            .map(|history| history.extrapolate_backward())
            .sum::<i64>();
        Ok(Box::new(sum))
    }
}

#[test]
fn test() {
    let input = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    // let input = "0 3 6 9 12 15";
    // assert_solution!(Day09.part_one, input, "114");
    assert_solution!(Day09.part_two, input, "2");
}
