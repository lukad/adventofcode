#![feature(int_roundings)]

mod year_2021;
mod year_2022;

use aoc::*;
use clap::Parser;
use std::{collections::HashMap, io::Read, path::PathBuf, str::FromStr, time::Instant};

#[derive(Debug, Parser)]
#[clap(author = "Luka Dornhecker")]
struct Opts {
    #[clap(short, long)]
    year: usize,
    #[clap(short, long)]
    day: usize,
    #[clap(short, long)]
    part: Part,
    #[clap(short, long)]
    bench: bool,
    input: PathBuf,
}

type Date = (usize, usize);

fn main() {
    let opts = Opts::parse();

    let mut solutions: HashMap<Date, _> = HashMap::new();

    for solution in inventory::iter::<&dyn Solution> {
        solutions.insert((solution.year(), solution.day()), solution);
    }

    let solution = solutions.get(&(opts.year, opts.day)).unwrap();

    let mut input = String::new();
    if opts.input == PathBuf::from_str("-").unwrap() {
        std::io::stdin().read_to_string(&mut input).unwrap();
    } else {
        input = std::fs::read_to_string(opts.input).unwrap();
    }

    let start = Instant::now();
    let output = solution.solve(&input, opts.part).unwrap();
    let took = Instant::now().duration_since(start);

    println!("{}", output);

    if opts.bench {
        eprintln!("Took {:?}", took);
    }
}
