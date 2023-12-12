use std::fmt;

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
    let map = build_map(lines);
    let start = find_start(&map);
    let mut pipes = get_adjacent_pipes(&start, &map);
    // Assumes that only two directions will be valid
    let (mut forward_dir, forward_pipe_type) = pipes.pop().unwrap();
    let (mut backwards_dir, backwards_pipe_type) = pipes.pop().unwrap();

    let forward_position = start
        .position
        .increment(&forward_dir)
        .expect("to be valid position");
    let mut forward_pipe = Pipe {
        pipe_type: forward_pipe_type,
        position: forward_position,
    };

    let backwards_position = start
        .position
        .increment(&backwards_dir)
        .expect("to be valid position");
    let mut backwards_pipe = Pipe {
        pipe_type: backwards_pipe_type,
        position: backwards_position,
    };

    // 3: Furthest point is when forwads and backwards meet again
    let mut distance = 1;
    while forward_pipe.position != backwards_pipe.position {
        forward_dir = forward_pipe.move_through(&forward_dir);
        forward_pipe.position = forward_pipe
            .position
            .increment(&forward_dir)
            .expect("to be valid position");
        forward_pipe.pipe_type =
            PipeType::from_map(&map, &forward_pipe.position).expect("to be valid pipe");

        backwards_dir = backwards_pipe.move_through(&backwards_dir);
        backwards_pipe.position = backwards_pipe
            .position
            .increment(&backwards_dir)
            .expect("to be valid position");
        backwards_pipe.pipe_type =
            PipeType::from_map(&map, &backwards_pipe.position).expect("to be valid pipe");

        distance += 1;
    }

    println!("Furthest point from start is {distance} away");
}

fn try_get_pipe_type(pipe: &Pipe, direction: &Direction, map: &[Vec<char>]) -> Option<PipeType> {
    if let Some(position) = &pipe.position.increment(direction) {
        PipeType::from_map(map, position)
    } else {
        None
    }
}

fn part2(lines: Vec<String>) {
    // Calculate interior points with Pick's theorem
    // loop_area = interior_points + (boundary_points / 2) - 1
    // rewriting, we get
    // interior_points = loop_area - (boundary_points / 2) - 1
    // We get loop_area with the Shoelace formula
    // border_points = (x1, y1), (x2, y2), (x3, y3), ...
    // 2 * loop_area = x1 * y2 - y1 * x2 + x2 * y3 - y2 * x3 + ...
    // loop_area = (result..) / 2
    let map = build_map(lines);
    let start = find_start(&map);
    let mut adjacent_pipes = get_adjacent_pipes(&start, &map);
    adjacent_pipes.pop();
    let (mut cursor_dir, cursor_type) = adjacent_pipes.pop().unwrap();

    let cursor_position = start
        .position
        .increment(&cursor_dir)
        .expect("to be valid position");
    let mut cursor_pipe = Pipe {
        pipe_type: cursor_type,
        position: cursor_position,
    };

    let mut border_points = vec![start.position.clone()];
    let mut vertices = vec![start.position.clone()];

    while cursor_pipe.position != start.position {
        border_points.push(cursor_pipe.position.clone());
        match cursor_pipe.pipe_type {
            PipeType::TurnF | PipeType::Turn7 | PipeType::TurnJ | PipeType::TurnL => {
                vertices.push(cursor_pipe.position.clone());
            }
            _ => (),
        }

        cursor_dir = cursor_pipe.move_through(&cursor_dir);
        cursor_pipe.position = cursor_pipe
            .position
            .increment(&cursor_dir)
            .expect("to be valid position");
        cursor_pipe.pipe_type =
            PipeType::from_map(&map, &cursor_pipe.position).expect("to be valid pipe");
    }

    let shoelace = vertices
        .windows(2)
        .map(|pair| {
            let (x1, y1) = (pair[0].col as i32, pair[0].row as i32);
            let (x2, y2) = (pair[1].col as i32, pair[1].row as i32);
            x1 * y2 - x2 * y1
        })
        .sum::<i32>();
    let last = vertices.last().unwrap();
    let first = vertices.first().unwrap();
    let shoelace = shoelace + ((last.col * first.row) as i32 - (last.row * first.col) as i32);

    let loop_area = i32::abs(shoelace) / 2;
    let interior_points = loop_area - (border_points.len() as i32 / 2) + 1;
    println!("There are {interior_points} inside the loop!");
}

fn build_map(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(map: &Vec<Vec<char>>) -> Pipe {
    let mut s = None;

    for (row, line) in map.iter().enumerate() {
        if s.is_some() {
            break;
        }
        for (col, c) in line.iter().enumerate() {
            if *c == 'S' {
                s = Some(Position::new(row, col));
                break;
            }
        }
    }

    Pipe::new(PipeType::TurnF, s.unwrap())
}

fn get_adjacent_pipes(source: &Pipe, map: &Vec<Vec<char>>) -> Vec<(Direction, PipeType)> {
    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .map(|direction| {
        (
            direction.clone(),
            try_get_pipe_type(source, &direction, map),
        )
    })
    .filter(|(_, pipe_type)| pipe_type.is_some())
    .map(|(direction, pipe_type)| (direction, pipe_type.unwrap()))
    .collect::<Vec<(Direction, PipeType)>>()
}

#[derive(Clone, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    fn increment(&self, direction: &Direction) -> Option<Position> {
        match (direction, self.row, self.col) {
            (Direction::North, 0, _col) => None,
            (Direction::North, _row, _col) => Some(Position::new(self.row - 1, self.col)),
            (Direction::South, row, col) => Some(Position::new(row + 1, col)),
            (Direction::East, row, col) => Some(Position::new(row, col + 1)),
            (Direction::West, _row, 0) => None,
            (Direction::West, _row, _col) => Some(Position::new(self.row, self.col - 1)),
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum PipeType {
    Vertical,
    Horizontal,
    TurnF,
    Turn7,
    TurnJ,
    TurnL,
    S,
}

impl PipeType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'F' => Some(Self::TurnF),
            '7' => Some(Self::Turn7),
            'J' => Some(Self::TurnJ),
            'L' => Some(Self::TurnL),
            'S' => Some(Self::S),
            _ => None,
        }
    }

    fn from_map(map: &[Vec<char>], position: &Position) -> Option<Self> {
        PipeType::from_char(map[position.row][position.col])
    }
}

#[derive(Debug)]
struct Pipe {
    pipe_type: PipeType,
    position: Position,
}

impl Pipe {
    fn new(pipe_type: PipeType, position: Position) -> Self {
        Pipe {
            pipe_type,
            position,
        }
    }

    fn move_through(&self, incoming_direction: &Direction) -> Direction {
        match (&self.pipe_type, incoming_direction) {
            (PipeType::Vertical, Direction::North) => Direction::North,
            (PipeType::Vertical, Direction::South) => Direction::South,
            (PipeType::Horizontal, Direction::East) => Direction::East,
            (PipeType::Horizontal, Direction::West) => Direction::West,
            (PipeType::TurnF, Direction::North) => Direction::East,
            (PipeType::TurnF, Direction::West) => Direction::South,
            (PipeType::Turn7, Direction::North) => Direction::West,
            (PipeType::Turn7, Direction::East) => Direction::South,
            (PipeType::TurnJ, Direction::East) => Direction::North,
            (PipeType::TurnJ, Direction::South) => Direction::West,
            (PipeType::TurnL, Direction::West) => Direction::North,
            (PipeType::TurnL, Direction::South) => Direction::East,
            (pipe, incoming_direction) => {
                panic!("Invalid move: enterting {pipe:?} from {incoming_direction:?}")
            }
        }
    }
}
