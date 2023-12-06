#![allow(dead_code)]

use crate::Part;

pub fn run(lines: Vec<String>, part: Part) {
    let before = std::time::Instant::now();
    match part {
        Part::One => part1(lines),
        Part::Two => part2(lines),
    }
    println!("Elapsed: {:.2?}", before.elapsed());
}

fn part1(lines: Vec<String>) {
    let mut seeds = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| (str::parse::<u64>(s).unwrap(), false))
        .collect::<Vec<(u64, bool)>>();

    for line in &lines[2..] {
        if !line.is_empty() && !line.contains("map") {
            let (dest_range_start, tail) = line.split_once(' ').unwrap();
            let (src_range_start, range_length) = tail.split_once(' ').unwrap();
            let dest_range_start = str::parse::<u64>(dest_range_start).unwrap();
            let src_range_start = str::parse::<u64>(src_range_start).unwrap();
            let range_length = str::parse::<u64>(range_length).unwrap();
            let src_range_end = src_range_start + range_length;

            seeds = seeds
                .iter()
                .map(|(seed, seen)| {
                    if !seen && *seed >= src_range_start && *seed <= src_range_end {
                        let inc = *seed - src_range_start;
                        let dest = dest_range_start + inc;
                        (dest, true)
                    } else {
                        (*seed, *seen)
                    }
                })
                .collect();
        } else {
            seeds = seeds.iter().map(|(seed, _)| (*seed, false)).collect()
        }
    }
    println!(
        "Minimum location: {}",
        seeds.iter().map(|(seed, _)| seed).min().unwrap()
    );
}

fn part2(_lines: Vec<String>) {}
