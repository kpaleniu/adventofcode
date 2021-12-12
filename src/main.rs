mod day1;
mod day2;
mod day3;

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Day1 {
        #[structopt(short, long)]
        windowed: bool,
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    Day2 {
        #[structopt(short, long)]
        part2: bool,
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    Day3 {
        #[structopt(short, long)]
        part2: bool,
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn d1(path: &PathBuf, windowed: bool) {
    let file = File::open(path);
    if let Ok(f) = file {
        let reader = BufReader::new(f);
        let measurements: Vec<u64> = reader
            .lines()
            .map(|m| m.unwrap().parse::<u64>().unwrap())
            .collect();
        if windowed {
            println!("increments {}", day1::windowed(&measurements));
        } else {
            println!("increments {}", day1::increments(&measurements));
        }
    }
}

fn d2(path: &PathBuf, part2: bool) {
    if let Ok(input) = fs::read_to_string(path) {
        if part2 {
            let (position, depth, aim) = day2::part2(&input);
            println!(
                "position {} depth {} aim {} ({})",
                position,
                depth,
                aim,
                position * depth
            );
        } else {
            let (position, depth) = day2::compute(&input);
            println!(
                "position {} depth {} ({})",
                position,
                depth,
                position * depth
            );
        }
    }
}

fn d3(path: &PathBuf, part2: bool) {
    if let Ok(input) = fs::read_to_string(path) {
        let radix = input.lines().nth(0).unwrap().len();
        if let Ok((gamma, epsilon)) = day3::part1(&input, radix) {
            println!(
                "{} (gamma: {}, epsilon: {})",
                gamma * epsilon,
                gamma,
                epsilon
            );
        }
    }
}

fn main() {
    let opt = Cli::from_args();
    match opt {
        Cli::Day1 { windowed, path } => d1(&path, windowed),
        Cli::Day2 { part2, path } => d2(&path, part2),
        Cli::Day3 { part2, path } => d3(&path, part2),
    }
}
