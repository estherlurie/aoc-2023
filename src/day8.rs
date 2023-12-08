use std::collections::HashMap;

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
    let map = get_map(&lines[2..]);
    let step_count = steps(&lines[0], "AAA".to_string(), &map);
    println!("Found ZZZ in {step_count} steps!");
}

fn part2(lines: Vec<String>) {
    let map = get_map(&lines[2..]);
    let totals = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(String::to_owned)
        .map(|start| steps(&lines[0], start, &map))
        .collect::<Vec<u64>>();
    let answer = totals.iter().fold(1, |x, steps| lcm(x, *steps));
    println!("Steps for all to be on Z: {answer}");
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn steps(direction_str: &str, start: String, map: &HashMap<String, Elements>) -> u64 {
    let mut node = start;
    let mut step_count = 0;
    for direction in direction_str.chars().map(Direction::from_char).cycle() {
        if node.ends_with("Z") {
            return step_count;
        } else {
            node = map.get(&node).unwrap().get(&direction).to_string();
            step_count += 1;
        }
    }
    0
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("Unknown direction '{c}'"),
        }
    }
}

struct Elements {
    left: String,
    right: String,
}

impl Elements {
    fn new(left: String, right: String) -> Self {
        Elements { left, right }
    }

    fn get(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn get_map(lines: &[String]) -> HashMap<String, Elements> {
    let mut map = HashMap::new();
    for line in lines {
        let (key, nodes) = line.split_once('=').unwrap();
        let key = key.trim().to_string();
        let (left, right) = nodes.trim().split_once(',').unwrap();
        let left = left.trim().strip_prefix("(").unwrap().to_string();
        let right = right.trim().strip_suffix(")").unwrap().to_string();
        map.insert(key, Elements::new(left, right));
    }
    map
}
