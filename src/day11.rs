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
    let map = get_map(lines);
    let galaxies: Vec<Galaxy> = get_galaxies(&map);

    let (rows_with_galaxies, cols_with_galaxies) = get_rows_and_cols_with_galaxies(&galaxies);

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_distance += true_distance(
                &galaxies[i],
                &galaxies[j],
                &rows_with_galaxies,
                &cols_with_galaxies,
                1,
            )
        }
    }
    println!("Total distance of shortest paths between all galaxies: {total_distance}");
}

fn part2(lines: Vec<String>) {
    let map = get_map(lines);
    let galaxies: Vec<Galaxy> = get_galaxies(&map);

    let (rows_with_galaxies, cols_with_galaxies) = get_rows_and_cols_with_galaxies(&galaxies);

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_distance += true_distance(
                &galaxies[i],
                &galaxies[j],
                &rows_with_galaxies,
                &cols_with_galaxies,
                999999,
            )
        }
    }
    println!("Total distance of shortest paths between all galaxies: {total_distance}");
}

struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn new(row: usize, col: usize) -> Self {
        Galaxy { row, col }
    }

    fn distance(&self, other: &Galaxy) -> usize {
        usize::abs_diff(self.row, other.row) + usize::abs_diff(self.col, other.col)
    }
}

fn get_map(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn get_galaxies(map: &[Vec<char>]) -> Vec<Galaxy> {
    map.iter()
        .enumerate()
        .flat_map(|(row, chars)| {
            chars
                .iter()
                .enumerate()
                .filter_map(move |(col, c)| {
                    if *c == '#' {
                        Some(Galaxy::new(row, col))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect()
}

fn get_rows_and_cols_with_galaxies(galaxies: &[Galaxy]) -> (HashSet<usize>, HashSet<usize>) {
    let rows_with_galaxies = galaxies.iter().map(|g| g.row).collect::<HashSet<usize>>();

    let cols_with_galaxies = galaxies.iter().map(|g| g.col).collect::<HashSet<usize>>();

    (rows_with_galaxies, cols_with_galaxies)
}

fn true_distance(
    this_galaxy: &Galaxy,
    other_galaxy: &Galaxy,
    rows_with_galaxies: &HashSet<usize>,
    cols_with_galaxies: &HashSet<usize>,
    expansion_factor: usize,
) -> usize {
    let mut total_distance = this_galaxy.distance(other_galaxy);

    for row in
        usize::min(this_galaxy.row, other_galaxy.row)..usize::max(this_galaxy.row, other_galaxy.row)
    {
        if !rows_with_galaxies.contains(&row) {
            total_distance += expansion_factor;
        }
    }

    for col in
        usize::min(this_galaxy.col, other_galaxy.col)..usize::max(this_galaxy.col, other_galaxy.col)
    {
        if !cols_with_galaxies.contains(&col) {
            total_distance += expansion_factor;
        }
    }

    total_distance
}
