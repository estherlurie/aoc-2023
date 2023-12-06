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
    let total_points = get_chosen_and_winning(lines)
        .iter()
        .map(|(chosen, winning)| {
            chosen.iter().fold(0, |points, c| {
                if winning.contains(c) {
                    if points == 0 {
                        1
                    } else {
                        points * 2
                    }
                } else {
                    points
                }
            })
        })
        .sum::<u32>();
    println!("Total points: {total_points}");
}

fn part2(lines: Vec<String>) {}

fn get_chosen_and_winning(lines: Vec<String>) -> Vec<(Vec<u32>, Vec<u32>)> {
    lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1.trim())
        .map(|points| {
            let (chosen, winning) = points.split_once('|').unwrap();
            (chosen.trim(), winning.trim())
        })
        .map(|(chosen, winning)| {
            (
                chosen
                    .split_ascii_whitespace()
                    .filter_map(|s| str::parse::<u32>(s).ok())
                    .collect::<Vec<u32>>(),
                winning
                    .split_ascii_whitespace()
                    .filter_map(|s| str::parse::<u32>(s).ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>()
}
