use aoc::*;
use ndarray::{s, Array2};

#[derive(Debug, Date)]
#[date(year = 2022, day = 8)]
pub struct Day08;

fn check_visibility(x: usize, y: usize, forest: &Array2<u32>, visibility: &mut Array2<u32>) {
    let tree = forest[[y, x]];
    let up = s![..y, x];
    let down = s![(y + 1).., x];
    let left = s![y, ..x];
    let right = s![y, (x + 1)..];
    if [up, down, left, right]
        .iter()
        .all(|slice| forest.slice(slice).iter().any(|other| *other >= tree))
    {
        visibility[[y, x]] = 0;
    }
}

fn parse_forest(input: &str) -> Result<Array2<u32>, ndarray::ShapeError> {
    let mut h = 0;
    let data: Vec<u32> = input
        .chars()
        .filter_map(|c| match c {
            '0'..='9' => Some(c as u32 - 48),
            _ => {
                h += 1;
                None
            }
        })
        .collect();

    let w = data.len() / h;

    let shape = (w, h);

    Array2::from_shape_vec(shape, data)
}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> AocResult {
        let forest = parse_forest(input).unwrap();
        let shape = forest.shape();
        let h = shape[0];
        let w = shape[1];
        let mut visibility = Array2::<u32>::ones([h, w]);

        for y in 1..(h - 1) {
            for x in 1..(w - 1) {
                check_visibility(x, y, &forest, &mut visibility);
            }
        }

        Ok(Box::new(visibility.sum()))
    }
}

#[test]
fn test() {
    let input = "30373\n25512\n65332\n33549\n35390\n";
    assert_solution!(Day08.part_one, input, "21");
    // assert_solution!(Day08.part_two, input, "8");
}
