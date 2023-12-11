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
    // 0: Build navigable char map
    let map = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // 1: Find start
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
    let start = Pipe::new(PipeType::TurnF, s.unwrap());

    // 2: Get forward and backwards pipes
    // Assumes that only two directions will be valid
    let mut pipes = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .map(|direction| {
        (
            direction.clone(),
            try_get_pipe_type(&start, &direction, &map),
        )
    })
    .filter(|(_, pipe_type)| pipe_type.is_some())
    .map(|(direction, pipe_type)| (direction, pipe_type.unwrap()))
    .collect::<Vec<(Direction, PipeType)>>();

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

fn part2(_lines: Vec<String>) {}

#[derive(Debug, PartialEq)]
struct Position {
    row: usize,
    col: usize,
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
