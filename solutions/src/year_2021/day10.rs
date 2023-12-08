use aoc::*;

fn check_parens(input: &str) -> (Vec<u64>, i32) {
    let mut stack = vec![];

    for c in input.chars() {
        match (c, stack.last()) {
            ('(', _) => stack.push(1),
            ('[', _) => stack.push(2),
            ('{', _) => stack.push(3),
            ('<', _) => stack.push(4),
            (')', Some(1)) => _ = stack.pop(),
            (']', Some(2)) => _ = stack.pop(),
            ('}', Some(3)) => _ = stack.pop(),
            ('>', Some(4)) => _ = stack.pop(),
            (')', _) => return (stack, 3),
            (']', _) => return (stack, 57),
            ('}', _) => return (stack, 1197),
            ('>', _) => return (stack, 25137),
            _ => unimplemented!(),
        }
    }

    stack.reverse();
    (stack, 0)
}

#[derive(Debug, Date)]
#[date(year = 2021, day = 10)]
pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> AocResult {
        let result = input.trim().lines().fold(0, |acc, line| {
            let (_incomplete, error_score) = check_parens(line);
            acc + error_score
        });
        Ok(Box::new(result))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let mut scores = input
            .trim()
            .lines()
            .filter_map(|line| {
                let (missing, error_score) = check_parens(line);
                if error_score == 0 {
                    Some(missing.into_iter().fold(0, |acc, x| acc * 5 + x))
                } else {
                    None
                }
            })
            .collect::<Vec<u64>>();

        scores.sort();

        Ok(Box::new(scores[scores.len() / 2]))
    }
}

#[test]
fn test() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    assert_solution!(Day10.part_one, input, "26397");
    assert_solution!(Day10.part_two, input, "288957");
}
