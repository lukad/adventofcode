mod year_2021;

use aoc::Solution;
use clap::Parser;
use std::{io::Read, str::FromStr};

#[derive(Debug)]
enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "one" => Ok(Self::One),
            "2" | "two" => Ok(Self::Two),
            _ => Err(format!("Unknown part {}", s)),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author = "Luka Dornhecker")]
struct Opts {
    #[clap(short, long)]
    year: i32,
    #[clap(short, long)]
    day: i32,
    #[clap(short, long)]
    part: Part,
}

fn main() {
    let opts = Opts::parse();

    let mut solutions: Vec<Box<dyn Solution>> = Vec::new();
    solutions.push(Box::new(year_2021::day01::solve_2021_01_01));
    solutions.push(Box::new(year_2021::day01::solve_2021_01_02));
    solutions.push(Box::new(year_2021::day02::solve_2021_02_02));
    solutions.push(Box::new(year_2021::day02::solve_2021_02_02));
    solutions.push(Box::new(year_2021::day03::solve_2021_03_01));
    solutions.push(Box::new(year_2021::day04::solve_2021_04_01));
    solutions.push(Box::new(year_2021::day04::solve_2021_04_02));
    solutions.push(Box::new(year_2021::day06::solve_2021_06_01));
    solutions.push(Box::new(year_2021::day10::solve_2021_10_01));
    solutions.push(Box::new(year_2021::day10::solve_2021_10_02));

    let part = match opts.part {
        Part::One => aoc::Part::One,
        Part::Two => aoc::Part::Two,
    };

    let solution = solutions.iter().find(|solution| {
        solution.day() == opts.day && solution.year() == opts.year && solution.part() == part
    });

    if let Some(solution) = solution {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        println!("{}", solution.solve(&input));
    } else {
        println!("No solution found");
    }
}
