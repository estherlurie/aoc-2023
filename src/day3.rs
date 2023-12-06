use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::Part;

pub fn run(lines: Vec<String>, part: Part) {
    match part {
        Part::One => part1(lines),
        Part::Two => part2(lines),
    }
}

fn part1(lines: Vec<String>) {
    // 1: Find all symbols with their coordinate
    let symbol_positions = get_symbol_positions(&lines);
    // 2: Find all numbers and list all adjacent coordinates
    let sum_of_part_numbers: usize = lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| get_part_numbers(row, line))
        .filter(|part_number| part_number.is_valid(&symbol_positions))
        .map(|part_number| part_number.id)
        .sum();
    println!("Sum of part numbers: {}", sum_of_part_numbers);
}

fn part2(lines: Vec<String>) {
    let mut parts_adjacent_to_gears = HashMap::<Position, Vec<PartNumber>>::new();
    let gear_positions = get_gear_positions(&lines);
    for (part_number, gear_position) in lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| get_part_numbers(row, line))
        .filter_map(|part_number| part_number.adjacent_gear_position(&gear_positions))
    {
        if let Some(adjacent_parts) = parts_adjacent_to_gears.get_mut(&gear_position) {
            adjacent_parts.push(part_number);
        } else {
            parts_adjacent_to_gears.insert(gear_position, vec![part_number]);
        }
    }

    let sum_of_gear_ratios: usize = parts_adjacent_to_gears
        .iter()
        .filter_map(|(_, adjacent_parts)| {
            if adjacent_parts.len() == 2 {
                Some(adjacent_parts[0].id * adjacent_parts[1].id)
            } else {
                None
            }
        })
        .sum();
    println!("Sum of gear ratios: {}", sum_of_gear_ratios);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug)]
struct PartNumber {
    id: usize,
    position: Position,
}

impl PartNumber {
    fn new(id: usize, position: Position) -> Self {
        PartNumber { id, position }
    }

    fn is_valid(&self, symbol_positions: &HashSet<Position>) -> bool {
        let adjacent_positions = self.get_adjacent_positions();
        for position in adjacent_positions {
            if symbol_positions.contains(&position) {
                return true;
            }
        }

        false
    }

    fn adjacent_gear_position(
        self,
        gear_positions: &HashSet<Position>,
    ) -> Option<(PartNumber, Position)> {
        let adjacent_positions = self.get_adjacent_positions();
        for position in adjacent_positions {
            if gear_positions.contains(&position) {
                return Some((self, position.clone()));
            }
        }
        None
    }

    fn get_adjacent_positions(&self) -> Vec<Position> {
        let mut adjacent_positions = Vec::<Position>::new();

        let len = self.id.to_string().len();
        let curr_row = self.position.row;
        let left_bound = if self.position.col != 0 {
            self.position.col - 1
        } else {
            self.position.col
        };
        let right_bound = self.position.col + len;

        // Check sides
        let left = Position::new(self.position.row, left_bound);
        let right = Position::new(self.position.row, right_bound);
        adjacent_positions.push(left);
        adjacent_positions.push(right);

        // Check below
        let mut rows_to_check = vec![curr_row + 1];
        if curr_row != 0 {
            // Check above
            rows_to_check.push(curr_row - 1);
        }

        for row in rows_to_check {
            for col in left_bound..=right_bound {
                adjacent_positions.push(Position::new(row, col));
            }
        }
        adjacent_positions
    }
}

impl fmt::Display for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part {} at {}", self.id, self.position)
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_alphanumeric() && c != '.'
}

fn get_symbol_positions(lines: &[String]) -> HashSet<Position> {
    let mut symbol_positions = HashSet::<Position>::new();
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if is_symbol(c) {
                let position = Position::new(row, col);
                symbol_positions.insert(position);
            }
        })
    });
    symbol_positions
}

fn get_gear_positions(lines: &[String]) -> HashSet<Position> {
    let mut gear_positions = HashSet::<Position>::new();
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c == '*' {
                let position = Position::new(row, col);
                gear_positions.insert(position);
            }
        })
    });
    gear_positions
}

fn get_part_numbers(row: usize, line: &str) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();
    let mut curr = 0;
    let mut position = None;

    for (col, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            if position.is_none() {
                position = Some(Position::new(row, col));
            }
            curr = curr * 10 + d;
        } else if curr != 0 {
            part_numbers.push(PartNumber::new(
                curr as usize,
                position.as_ref().unwrap().clone(),
            ));

            curr = 0;
            position = None;
        }
    }

    part_numbers
}
