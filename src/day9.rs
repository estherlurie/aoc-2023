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
    let mut answer = 0;
    for values in lines.iter().map(String::as_str).map(to_vec) {
        answer += last_of_sequence_diffs(vec![*values.last().unwrap()], &values)
            .iter()
            .sum::<i32>();
    }
    println!("Sum of next sequence values: {answer}");
}

fn part2(lines: Vec<String>) {
    let mut answer = 0;
    for values in lines.iter().map(String::as_str).map(to_vec) {
        answer += prev_of_sequence_diffs(vec![*values.first().unwrap()], &values)
            .iter()
            .rev()
            .fold(0, |prev_in_sequence, first| first - prev_in_sequence);
    }
    println!("Sum of prev sequence values: {answer}");
}

fn to_vec(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .filter_map(|s| str::parse::<i32>(s).ok())
        .collect()
}

fn last_of_sequence_diffs(mut last_of_each_sequence: Vec<i32>, values: &[i32]) -> Vec<i32> {
    if values.iter().all(|v| *v == 0) {
        last_of_each_sequence
    } else {
        let mut diffs = vec![];
        let mut idx = 1;
        while idx < values.len() {
            diffs.push(values[idx] - values[idx - 1]);
            idx += 1;
        }
        if let Some(last) = diffs.last() {
            last_of_each_sequence.push(*last);
        }
        last_of_sequence_diffs(last_of_each_sequence, &diffs)
    }
}

fn prev_of_sequence_diffs(mut first_of_each_sequence: Vec<i32>, values: &[i32]) -> Vec<i32> {
    if values.iter().all(|v| *v == 0) {
        first_of_each_sequence
    } else {
        let mut diffs = vec![];
        let mut idx = 1;
        while idx < values.len() {
            let diff = values[idx] - values[idx - 1];
            diffs.push(diff);
            idx += 1;
        }
        if let Some(first) = diffs.first() {
            first_of_each_sequence.push(*first);
        }
        prev_of_sequence_diffs(first_of_each_sequence, &diffs)
    }
}
