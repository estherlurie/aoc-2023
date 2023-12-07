use num_format::{Locale, ToFormattedString};

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

fn part2(lines: Vec<String>) {
    // Maybe I can work backwards? Nah... just testing numbers until I find the right one? Unless?
    let seeds = get_seed_ranges(&lines[0]);

    type Maps = Vec<ReverseMap>;
    let seed_to_soil = Maps::new();
    let soil_to_fertilizer = Maps::new();
    let fetrilizer_to_water = Maps::new();
    let water_to_light = Maps::new();
    let light_to_temperature = Maps::new();
    let temperature_to_humidity = Maps::new();
    let humidity_to_location = Maps::new();

    let mut maps = [
        humidity_to_location,
        temperature_to_humidity,
        light_to_temperature,
        water_to_light,
        fetrilizer_to_water,
        soil_to_fertilizer,
        seed_to_soil,
    ];

    let mut map_idx = 6;
    for line in &lines[2..] {
        if line.contains("map") {
            continue;
        } else if line.is_empty() {
            map_idx -= 1;
            continue;
        }
        let curr_maps = &mut maps[map_idx];
        let (dst_range_start, tail) = line.split_once(' ').unwrap();
        let (src_range_start, len) = tail.trim().split_once(' ').unwrap();
        let dst_range_start = str::parse::<u64>(dst_range_start).unwrap();
        let src_range_start = str::parse::<u64>(src_range_start).unwrap();
        let len = str::parse::<u64>(len).unwrap();
        let reverse_map = ReverseMap::new(dst_range_start, src_range_start, len);
        curr_maps.push(reverse_map);
    }

    for location in 0u64.. {
        if location % 100000 == 0 {
            println!("Checking {}", location.to_formatted_string(&Locale::en));
        }
        let mut n = location;
        for map in &maps {
            if let Some(m) = map.iter().fold(None, |maybe, curr_map| {
                if maybe.is_some() {
                    maybe
                } else {
                    curr_map.map_if_contains(n)
                }
            }) {
                n = m;
            }
        }
        for seed in &seeds {
            if seed.contains(n) {
                println!("Location found: {location}");
                return;
            }
        }
    }
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    fn new(start: u64, len: u64) -> Self {
        SeedRange {
            start,
            end: start + len,
        }
    }

    fn contains(&self, n: u64) -> bool {
        n >= self.start && n <= self.end
    }
}

#[derive(Debug)]
struct ReverseMap {
    from_start: u64,
    from_end: u64,
    to_start: u64,
}

impl ReverseMap {
    fn new(from: u64, to: u64, len: u64) -> Self {
        ReverseMap {
            from_start: from,
            from_end: from + len,
            to_start: to,
        }
    }

    fn map_if_contains(&self, n: u64) -> Option<u64> {
        if n >= self.from_start && n <= self.from_end {
            let inc = n - self.from_start;
            Some(self.to_start + inc)
        } else {
            None
        }
    }
}

fn get_seed_ranges(line: &str) -> Vec<SeedRange> {
    line.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| (str::parse::<u64>(s).unwrap()))
        .enumerate()
        .fold(
            Vec::<Vec<u64>>::new(),
            |mut seed_pairs, (idx, seed_or_range_len)| {
                if idx % 2 == 0 {
                    // Seed start
                    seed_pairs.push(vec![seed_or_range_len]);
                } else {
                    // Range len
                    seed_pairs.last_mut().unwrap().push(seed_or_range_len);
                }
                seed_pairs
            },
        )
        .iter()
        .fold(Vec::<SeedRange>::new(), |mut seeds, seed_range_pair| {
            seeds.push(SeedRange::new(seed_range_pair[0], seed_range_pair[1]));
            seeds
        })
}
