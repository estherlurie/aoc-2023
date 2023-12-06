mod day1;
mod day2;
mod template;

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
        3 => panic!("Day not yet implemented"),
        4 => panic!("Day not yet implemented"),
        5 => panic!("Day not yet implemented"),
        6 => panic!("Day not yet implemented"),
        7 => panic!("Day not yet implemented"),
        8 => panic!("Day not yet implemented"),
        9 => panic!("Day not yet implemented"),
        10 => panic!("Day not yet implemented"),
        11 => panic!("Day not yet implemented"),
        12 => panic!("Day not yet implemented"),
        13 => panic!("Day not yet implemented"),
        14 => panic!("Day not yet implemented"),
        15 => panic!("Day not yet implemented"),
        16 => panic!("Day not yet implemented"),
        17 => panic!("Day not yet implemented"),
        18 => panic!("Day not yet implemented"),
        19 => panic!("Day not yet implemented"),
        20 => panic!("Day not yet implemented"),
        21 => panic!("Day not yet implemented"),
        22 => panic!("Day not yet implemented"),
        23 => panic!("Day not yet implemented"),
        24 => panic!("Day not yet implemented"),
        25 => panic!("Day not yet implemented"),
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
