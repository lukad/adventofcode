use aoc::aoc;

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

#[aoc(year = 2021, day = 10, part = "one")]
pub fn solve_2021_10_01(input: &str) -> String {
    Box::new(input.trim().lines().into_iter().fold(0, |acc, line| {
        let (_incomplete, error_score) = check_parens(line);
        acc + error_score
    }))
}

#[aoc(year = 2021, day = 10, part = "two")]
fn solve_2021_10_02(input: &str) -> Box<u64> {
    let mut scores = input
        .trim()
        .lines()
        .into_iter()
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

    Box::new(scores[scores.len() / 2])
}

#[test]
fn test() {
    use aoc::Solution;
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
    assert_eq!(
        solve_2021_10_01.solve(input).to_string(),
        "26397".to_string()
    );
    assert_eq!(
        solve_2021_10_02.solve(input).to_string(),
        "288957".to_string()
    );
}
