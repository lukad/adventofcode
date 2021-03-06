mod year_2021;

use aoc::Solution;
use clap::Parser;
use std::{io::Read, str::FromStr, time::Instant};

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
    #[clap(short, long)]
    bench: bool,
}

fn main() {
    let opts = Opts::parse();

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(year_2021::day01::solve_2021_01_01),
        Box::new(year_2021::day01::solve_2021_01_02),
        Box::new(year_2021::day02::solve_2021_02_02),
        Box::new(year_2021::day02::solve_2021_02_02),
        Box::new(year_2021::day03::solve_2021_03_01),
        Box::new(year_2021::day04::solve_2021_04_01),
        Box::new(year_2021::day04::solve_2021_04_02),
        Box::new(year_2021::day06::solve_2021_06_01),
        Box::new(year_2021::day06::solve_2021_06_02),
        Box::new(year_2021::day07::solve_2021_07_01),
        Box::new(year_2021::day07::solve_2021_07_02),
        Box::new(year_2021::day10::solve_2021_10_01),
        Box::new(year_2021::day10::solve_2021_10_02),
        Box::new(year_2021::day09::solve_2021_09_01),
        Box::new(year_2021::day09::solve_2021_09_02),
        Box::new(year_2021::day11::solve_2021_11_01),
        Box::new(year_2021::day11::solve_2021_11_02),
        Box::new(year_2021::day12::solve_2021_12_01),
        Box::new(year_2021::day12::solve_2021_12_02),
    ];

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
        let start = Instant::now();
        let output = solution.solve(&input);
        let took = Instant::now().duration_since(start);

        println!("{}", output);
        if opts.bench {
            eprintln!("Took {:?}", took);
        }
    } else {
        println!("No solution found");
    }
}
