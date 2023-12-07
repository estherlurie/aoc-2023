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
    let races = get_races(lines);
    let answer = races
        .iter()
        .map(|race| race.number_ways_to_beat_record())
        .product::<u64>();
    println!("Product of number of ways to beat each race: {answer}");
}

fn part2(_lines: Vec<String>) {}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn new(time: u64, record: u64) -> Self {
        Race { time, record }
    }

    fn number_ways_to_beat_record(&self) -> u64 {
        // calculate minimum time
        let mut time_holding_button = 0;
        let mut dist = time_holding_button * (self.time - time_holding_button);
        while dist <= self.record {
            time_holding_button += 1;
            dist = time_holding_button * (self.time - time_holding_button);
        }
        let minimum_time_holding_button = time_holding_button;

        // calculate max time
        time_holding_button = self.time;
        dist = time_holding_button * (self.time - time_holding_button);
        while dist <= self.record {
            time_holding_button -= 1;
            dist = time_holding_button * (self.time - time_holding_button);
        }
        let maximum_time_holding_button = time_holding_button;

        1 + maximum_time_holding_button - minimum_time_holding_button
    }
}

fn get_races(lines: Vec<String>) -> Vec<Race> {
    parse_nums(&lines[0])
        .iter()
        .zip(&mut parse_nums(&lines[1]))
        .map(|(time, record)| Race::new(*time, *record))
        .collect()
}

fn parse_nums(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .map(|s| str::parse::<u64>(s).unwrap())
        .collect()
}
