use std::collections::HashSet;

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
    let mut map = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let galaxies: Vec<Galaxy> = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| {
                    if c == '#' {
                        Some(Galaxy::new(Point::new(row, col)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Galaxy>>()
        })
        .flatten()
        .collect();

    let rows_with_galaxies = galaxies
        .iter()
        .map(|g| g.position.row)
        .collect::<HashSet<usize>>();

    let cols_with_galaxies = galaxies
        .iter()
        .map(|g| g.position.col)
        .collect::<HashSet<usize>>();

    let row_len = lines[0].len();
    let mut rows_inserted = 0;
    for (row, _) in lines.iter().enumerate() {
        if !rows_with_galaxies.contains(&row) {
            map.insert(row + rows_inserted, vec!['.'; row_len]);
            rows_inserted += 1;
        }
    }

    for row_idx in 0..map.len() {
        let row = &mut map[row_idx];
        let mut cols_inserted = 0;
        for col_idx in 0..row.len() {
            if !cols_with_galaxies.contains(&col_idx) {
                row.insert(col_idx + cols_inserted, '.');
                cols_inserted += 1;
            }
        }
    }

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_distance += galaxies[i].distance(&galaxies[j]);
        }
    }
    println!("Total distance of shortest paths between all galaxies: {total_distance}");
}

fn part2(_lines: Vec<String>) {}

struct Galaxy {
    position: Point,
}

impl Galaxy {
    fn new(position: Point) -> Self {
        Galaxy { position }
    }

    fn distance(&self, other: &Galaxy) -> usize {
        self.position.distance(&other.position)
    }
}

struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }

    fn distance(&self, other: &Point) -> usize {
        usize::abs_diff(self.row, other.row) + usize::abs_diff(self.col, other.col)
    }
}
