use aoc::*;
use parse_display::{Display, FromStr};

#[derive(Display, Debug, FromStr, PartialEq, Eq, Clone, Copy)]
enum Ins {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(i32),
}

impl Ins {
    fn cycles(&self) -> usize {
        match self {
            Ins::Noop => 1,
            Ins::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Cpu {
    x: i32,
    cycles: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            cycles: Default::default(),
        }
    }
}

impl Cpu {
    fn new() -> Self {
        Default::default()
    }

    fn run<I>(&mut self, instructions: I, callback: &mut dyn FnMut(&mut Self))
    where
        I: Iterator<Item = Ins>,
    {
        for ins in instructions {
            for _ in 0..ins.cycles() {
                self.cycles += 1;
                callback(self);
            }

            match ins {
                Ins::Noop => (),
                Ins::Addx(n) => self.x += n,
            }
        }
    }
}

#[derive(Debug, Date)]
#[date(year = 2022, day = 10)]
pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> AocResult {
        let instructions = input.lines().filter_map(|line| line.parse::<Ins>().ok());

        let mut cpu = Cpu::new();
        let mut sum = 0;
        cpu.run(instructions, &mut |cpu| {
            if cpu.cycles == 20 || (cpu.cycles > 20 && (cpu.cycles - 20) % 40 == 0) {
                let ssi = cpu.x * cpu.cycles as i32;
                sum += ssi;
            }
        });

        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let instructions = input.lines().filter_map(|line| line.parse::<Ins>().ok());

        let mut out = vec![];
        let mut line = vec![];
        let mut cpu = Cpu::new();
        cpu.run(instructions, &mut |cpu| {
            let pos = (cpu.cycles as i32 - 1) % 40;
            if cpu.x - 1 <= pos && cpu.x + 1 >= pos {
                line.push('#');
            } else {
                line.push('.');
            }
            if cpu.cycles > 0 && cpu.cycles % 40 == 0 {
                out.push(line.clone());
                line.clear();
            }
        });

        let output = out
            .into_iter()
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>()
            .join("\n");

        Ok(Box::new(output))
    }
}

#[test]
fn test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_solution!(Day10.part_one, input, "13140");
    assert_solution!(
        Day10.part_two,
        input,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}
