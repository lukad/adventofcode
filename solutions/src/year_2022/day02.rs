use aoc::*;

pub trait SignedModulo {
    fn modulo(&self, n: Self) -> Self;
}

macro_rules! impl_signed_modulo {
    ($($t:ty)*) => ($(
      impl SignedModulo for $t {
        fn modulo(&self, n: Self) -> Self {
          let r = self % n;
          if r < 0 {
            r + n
          } else {
            r
          }
        }
      }
    )*)
}

impl_signed_modulo! { i8 i16 i32 i64 }

#[derive(Debug, Date)]
#[date(year = 2022, day = 2)]
pub struct Day02;

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> AocResult {
        Ok(Box::new(
            input.chars().collect::<Vec<_>>()[..]
                .chunks(4)
                .map(|x| (x[0] as i32 & 3, x[2] as i32 & 3))
                .map(|(a, b)| b + 1 + (b - a + 2).modulo(3) * 3)
                .sum::<i32>(),
        ))
    }

    fn part_two(&self, input: &str) -> AocResult {
        Ok(Box::new(
            input.chars().collect::<Vec<_>>()[..]
                .chunks(4)
                .map(|x| (x[0] as i32 & 3, x[2] as i32 & 3))
                .map(|(a, b)| 1 + 3 * b + (a + b - 2).modulo(3))
                .sum::<i32>(),
        ))
    }
}

#[test]
fn test() {
    let input = "A Y
B X
C Z
";
    assert_solution!(Day02.part_one, input, "15");
    assert_solution!(Day02.part_two, input, "12");
}
