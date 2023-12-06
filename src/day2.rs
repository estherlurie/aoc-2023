use crate::Part;

pub fn run(lines: Vec<String>, part: Part) {
    match part {
        Part::One => part1(lines),
        Part::Two => part2(lines),
    }
}

fn part1(lines: Vec<String>) {
    let (red_cubes, green_cubes, blue_cubes) = (12, 13, 14);
    let sum_of_possible_game_ids: u32 = lines
        .iter()
        .enumerate()
        .map(|(idx, line)| parse_game(idx as u32 + 1, line))
        .filter(|game| game.is_possible(red_cubes, green_cubes, blue_cubes))
        .map(|game| game.round)
        .sum();
    println!("Sum of possible game IDs: {sum_of_possible_game_ids}");
}

fn part2(lines: Vec<String>) {
    let sum_of_power: u32 = lines
        .iter()
        .enumerate()
        .map(|(idx, line)| parse_game(idx as u32 + 1, line))
        .map(Game::minimum_cubes_needed)
        .map(|cube_count| cube_count.0 * cube_count.1 * cube_count.2)
        .sum();
    println!("Sum of power of minimum cubes needed: {sum_of_power}");
}

struct Game {
    round: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self, red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> bool {
        self.draws
            .iter()
            .all(|round| round.is_possible(red_cubes, green_cubes, blue_cubes))
    }

    fn minimum_cubes_needed(game: Game) -> (u32, u32, u32) {
        game.draws
            .iter()
            .map(Draw::minimum_cubes_needed)
            .fold((0, 0, 0), |acc, draw| {
                (
                    u32::max(acc.0, draw.0),
                    u32::max(acc.1, draw.1),
                    u32::max(acc.2, draw.2),
                )
            })
        /*
            .fold(
            (0, 0, 0),
            |((prev_red, prev_green, prev_blue), (red, green, blue))| {
                (
                    u32::max(prev_red, red),
                    u32::max(prev_green, green),
                    u32::max(prev_blue, blue),
                )
            },
        )
            */
    }
}

struct Draw {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Draw {
    fn empty() -> Self {
        Draw {
            red: None,
            green: None,
            blue: None,
        }
    }

    fn is_possible(&self, red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> bool {
        self.red.unwrap_or(0) <= red_cubes
            && self.green.unwrap_or(0) <= green_cubes
            && self.blue.unwrap_or(0) <= blue_cubes
    }

    fn minimum_cubes_needed(&self) -> (u32, u32, u32) {
        (
            self.red.unwrap_or(0),
            self.green.unwrap_or(0),
            self.blue.unwrap_or(0),
        )
    }
}

fn parse_game(round: u32, line: &str) -> Game {
    let draws = line
        .split_once(':')
        .unwrap()
        .1
        .split(';')
        .map(str::trim)
        .map(parse_draw)
        .collect();
    Game { round, draws }
}

fn parse_draw(line: &str) -> Draw {
    let mut draw = Draw::empty();
    for single_draw in line.split(',') {
        let (count_str, color_str) = single_draw.trim().split_once(' ').unwrap();
        let count = str::parse::<u32>(count_str).unwrap();
        match color_str {
            "red" => draw.red = Some(count),
            "green" => draw.green = Some(count),
            "blue" => draw.blue = Some(count),
            _ => unreachable!(),
        }
    }
    draw
}
