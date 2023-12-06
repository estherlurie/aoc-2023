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
    let calibration_sum: u32 = lines.iter().map(String::as_str).map(parse_digit_line).sum();
    println!("Calibration sum: {calibration_sum}");
}

fn parse_digit_line(line: &str) -> u32 {
    let tens: u32 = {
        let mut r = 0;
        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                r = n * 10;
                break;
            }
        }
        r
    };
    let ones: u32 = {
        let mut r = 0;
        for c in line.chars().rev() {
            if let Some(n) = c.to_digit(10) {
                r = n;
                break;
            }
        }
        r
    };
    tens + ones
}

fn part2(lines: Vec<String>) {
    let calibration_sum: u32 = lines
        .iter()
        .map(String::as_str)
        .map(transform_line)
        .map(|s| parse_digit_line(&s))
        .sum();
    println!("Calibration sum: {calibration_sum}");
}

fn transform_line(line: &str) -> String {
    line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}
