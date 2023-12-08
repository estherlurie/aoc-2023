mod template;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: Part,

    #[arg(short, long)]
    test: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum Part {
    One,
    Two,
}

fn main() {
    let args = Args::parse();
    let filename = format!(
        "input/day{}{}.txt",
        args.day,
        if args.test { "_test" } else { "" },
    );
    let lines = get_input(&filename);
    match args.day {
        1 => day1::run(lines, args.part),
        2 => day2::run(lines, args.part),
        3 => day3::run(lines, args.part),
        4 => day4::run(lines, args.part),
        5 => day5::run(lines, args.part),
        6 => day6::run(lines, args.part),
        7 => day7::run(lines, args.part),
        8 => day8::run(lines, args.part),
        9..=25 => panic!("Day not yet implemented"),
        _ => panic!("Entered a day that has not yet been implemented"),
    }
}

fn get_input(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}
