fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapPiece {
    Start,
    Ground,
    Segment(Direction, Direction),
}

impl From<char> for MapPiece {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Ground,
            '|' => Self::Segment(Direction::Up, Direction::Down),
            '-' => Self::Segment(Direction::Left, Direction::Right),
            'L' => Self::Segment(Direction::Up, Direction::Right),
            'J' => Self::Segment(Direction::Up, Direction::Left),
            '7' => Self::Segment(Direction::Left, Direction::Down),
            'F' => Self::Segment(Direction::Right, Direction::Down),
            _ => panic!("Invalid segment type {}", value),
            //yes, this is bad and wrong, lazy for toy problem
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn move_in_dir(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Self {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

struct WalkerState {
    position: Location,
    moved_from: Direction,
}

impl WalkerState {
    fn new(position: Location, moved_from: Direction) -> Self {
        Self {
            position,
            moved_from,
        }
    }

    fn step(&mut self, square: &MapPiece) {
        match square {
            MapPiece::Start => panic!("Reached start square in step"),
            MapPiece::Ground => panic!("Reach Ground square in step"),
            MapPiece::Segment(dir1, dir2) => {
                if self.moved_from.reverse() == *dir1 {
                    self.moved_from = *dir2;
                    self.position = self.position.move_in_dir(dir2);
                } else {
                    self.moved_from = *dir1;
                    self.position = self.position.move_in_dir(dir1);
                }
            }
        }
    }

    fn met(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

struct SolveState {
    map: Vec<Vec<MapPiece>>,
    _start: Location,
    path1: WalkerState,
    path2: WalkerState,
    steps: usize,
}

impl SolveState {
    fn from_input(input: &str) -> Self {
        let map: Vec<Vec<MapPiece>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        let mut start = Location::new(0, 0);
        for (row_n, row) in map.iter().enumerate() {
            for (col_n, c) in row.iter().enumerate() {
                if *c == MapPiece::Start {
                    start = Location::new(row_n, col_n);
                }
            }
        }

        let mut path1 = None;
        let mut path2 = None;

        for dir in [
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Up,
        ] {
            let check_step = start.move_in_dir(&dir);
            if let MapPiece::Segment(dir1, dir2) = map[check_step.row][check_step.col] {
                if dir1 == dir.reverse() || dir2 == dir.reverse() {
                    let state = WalkerState::new(check_step, dir);
                    if path1.is_none() {
                        path1 = Some(state);
                    } else {
                        path2 = Some(state);
                        break; // filled both
                    }
                }
            }
        }

        Self {
            map,
            _start: start,
            path1: path1.unwrap(),
            path2: path2.unwrap(),
            steps: 1,
        }
    }

    fn lookup(&self, loc: &Location) -> MapPiece {
        self.map[loc.row][loc.col]
    }

    fn is_done(&self) -> bool {
        self.path1.met(&self.path2)
    }

    fn step_both(&mut self) -> bool {
        self.path1.step(&self.lookup(&self.path1.position));
        self.path2.step(&self.lookup(&self.path2.position));

        self.steps += 1;
        self.is_done()
    }
}

fn part1(text: &str) {
    let mut state = SolveState::from_input(text);
    while !state.step_both() {}

    println!("{}", state.steps);
}

fn part2(text: &str) {}
