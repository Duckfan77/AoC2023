use std::collections::HashMap;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::South => Self::North,
            Direction::West => Self::East,
            Direction::East => Self::West,
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }

    fn anticlockwise(&self) -> Self {
        self.clockwise().reverse()
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
            '|' => Self::Segment(Direction::North, Direction::South),
            '-' => Self::Segment(Direction::West, Direction::East),
            'L' => Self::Segment(Direction::North, Direction::East),
            'J' => Self::Segment(Direction::North, Direction::West),
            '7' => Self::Segment(Direction::West, Direction::South),
            'F' => Self::Segment(Direction::East, Direction::South),
            _ => panic!("Invalid segment type {}", value),
            //yes, this is bad and wrong, lazy for toy problem
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    row: isize,
    col: isize,
}

impl Location {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn move_in_dir(&self, dir: &Direction) -> Self {
        match dir {
            Direction::North => Self {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::West => Self {
                row: self.row,
                col: self.col - 1,
            },
            Direction::East => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }

    fn adjacents(&self) -> impl Iterator<Item = Self> + '_ {
        [
            Direction::West,
            Direction::South,
            Direction::East,
            Direction::North,
        ]
        .iter()
        .map(|dir| self.move_in_dir(dir))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnclosedStatus {
    Unknown,
    Loop,
    RightSide,
    LeftSide,
}

struct SolveState {
    map: HashMap<Location, (MapPiece, EnclosedStatus)>,
    start: Location,
    path1: WalkerState,
    path2: WalkerState,
    steps: usize,
    inside: Option<EnclosedStatus>,
    row_count: isize,
    col_count: isize,
}

impl SolveState {
    fn from_input(input: &str) -> Self {
        let mut map: HashMap<Location, (MapPiece, EnclosedStatus)> = input
            .lines()
            .enumerate()
            .flat_map(|(row_n, line)| {
                line.char_indices().map(move |(col_n, c)| {
                    (
                        Location::new(row_n as isize, col_n as isize),
                        (c.into(), EnclosedStatus::Unknown),
                    )
                })
            })
            .collect();

        let mut start = Location::new(0, 0);
        for (loc, (c, status)) in map.iter_mut() {
            if *c == MapPiece::Start {
                start = loc.clone();
                *status = EnclosedStatus::Loop;
            }
        }

        let mut path1 = None;
        let mut path2 = None;

        for dir in [
            Direction::West,
            Direction::South,
            Direction::East,
            Direction::North,
        ] {
            let check_step = start.move_in_dir(&dir);
            if let MapPiece::Segment(dir1, dir2) = map.get(&check_step).unwrap().0 {
                if dir1 == dir.reverse() || dir2 == dir.reverse() {
                    let state = WalkerState::new(check_step, dir);
                    if path1.is_none() {
                        path1 = Some(state);
                        map.entry(check_step)
                            .and_modify(|(_, status)| *status = EnclosedStatus::Loop);
                    } else {
                        path2 = Some(state);
                        map.entry(check_step)
                            .and_modify(|(_, status)| *status = EnclosedStatus::Loop);
                        break; // filled both
                    }
                }
            }
        }

        map.entry(start).and_modify(|(c, _)| {
            *c = MapPiece::Segment(
                path1.as_ref().unwrap().moved_from,
                path2.as_ref().unwrap().moved_from,
            )
        });

        Self {
            map,
            start,
            path1: path1.unwrap(),
            path2: path2.unwrap(),
            steps: 1,
            inside: None,
            row_count: input.lines().count() as isize,
            col_count: input.lines().next().unwrap().chars().count() as isize,
        }
    }

    fn lookup_piece(&self, loc: &Location) -> MapPiece {
        self.map.get(loc).unwrap().0
    }

    fn is_done(&self) -> bool {
        self.path1.met(&self.path2)
    }

    fn step_both(&mut self) -> bool {
        self.path1.step(&self.lookup_piece(&self.path1.position));
        self.path2.step(&self.lookup_piece(&self.path2.position));

        self.map
            .entry(self.path1.position)
            .and_modify(|(_, status)| *status = EnclosedStatus::Loop);

        self.map
            .entry(self.path2.position)
            .and_modify(|(_, status)| *status = EnclosedStatus::Loop);

        self.steps += 1;
        self.is_done()
    }

    fn at_start(&self) -> bool {
        self.path1.position == self.start
    }

    fn step_path1(&mut self) -> bool {
        self.path1.step(&self.lookup_piece(&self.path1.position));

        self.map
            .entry(self.path1.position)
            .and_modify(|(_, status)| *status = EnclosedStatus::Loop);

        self.at_start()
    }

    fn mark_loop(&mut self) {
        while !self.step_path1() {}
    }

    fn tag_path1_adjacent_cells(&mut self) {
        let loc = self.path1.position;
        if let MapPiece::Segment(mut dir1, mut dir2) = self.lookup_piece(&loc) {
            if self.path1.moved_from.reverse() == dir2 {
                // came from dir 2, swap them for this
                let temp = dir1;
                dir1 = dir2;
                dir2 = temp;
            }

            if dir1.clockwise() == dir2 {
                // turns right, tag the other two as LeftSide
                self.map
                    .entry(loc.move_in_dir(&dir1.reverse()))
                    .and_modify(|(_, status)| {
                        if *status == EnclosedStatus::Unknown {
                            *status = EnclosedStatus::LeftSide
                        }
                    });
                self.map
                    .entry(loc.move_in_dir(&dir1.anticlockwise()))
                    .and_modify(|(_, status)| {
                        if *status == EnclosedStatus::Unknown {
                            *status = EnclosedStatus::LeftSide
                        }
                    });
            } else if dir1.anticlockwise() == dir2 {
                // turns left, tag the other two as RightSide
                self.map
                    .entry(loc.move_in_dir(&dir1.reverse()))
                    .and_modify(|(_, status)| {
                        if *status == EnclosedStatus::Unknown {
                            *status = EnclosedStatus::RightSide
                        }
                    });
                self.map
                    .entry(loc.move_in_dir(&dir1.clockwise()))
                    .and_modify(|(_, status)| {
                        if *status == EnclosedStatus::Unknown {
                            *status = EnclosedStatus::RightSide
                        }
                    });
            } else {
                // goes straight, tag the other two as left and right
                self.map
                    .entry(loc.move_in_dir(&dir1.clockwise()))
                    .and_modify(|(_, status)| {
                        if *status == EnclosedStatus::Unknown {
                            *status = EnclosedStatus::RightSide
                        }
                    });
                self.map
                    .entry(loc.move_in_dir(&dir1.anticlockwise()))
                    .and_modify(|(_, status)| {
                        if *status == EnclosedStatus::Unknown {
                            *status = EnclosedStatus::LeftSide
                        }
                    });
            }
        }
    }

    fn tag_loop_adjacent_cells(&mut self) {
        self.tag_path1_adjacent_cells();
        self.step_path1();
        while !self.at_start() {
            self.tag_path1_adjacent_cells();
            self.step_path1();
        }
    }

    /// does a single step of flooding the input, returns true
    /// if any cells were changed
    fn single_flood_step(&mut self) -> bool {
        let mut cells_changed = 0;

        for row_n in 0..self.row_count {
            for col_n in 0..self.col_count {
                let spread_from = Location::new(row_n, col_n);
                let cell_type = self.map.get(&spread_from).unwrap().1;
                if cell_type == EnclosedStatus::LeftSide || cell_type == EnclosedStatus::RightSide {
                    for adj in spread_from.adjacents() {
                        match self.map.get_mut(&adj) {
                            Some((_, status)) => {
                                if *status == EnclosedStatus::Unknown {
                                    *status = cell_type;
                                    cells_changed += 1;
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        cells_changed != 0
    }

    fn flood_sides(&mut self) {
        while self.single_flood_step() {}
    }

    fn find_outside_type(&self) -> EnclosedStatus {
        for col in 0..self.col_count {
            match self.map.get(&Location { row: 0, col: col }).unwrap().1 {
                EnclosedStatus::RightSide => return EnclosedStatus::RightSide,
                EnclosedStatus::LeftSide => return EnclosedStatus::LeftSide,
                _ => {}
            }

            match self
                .map
                .get(&Location {
                    row: self.row_count - 1,
                    col: col,
                })
                .unwrap()
                .1
            {
                EnclosedStatus::RightSide => return EnclosedStatus::RightSide,
                EnclosedStatus::LeftSide => return EnclosedStatus::LeftSide,
                _ => {}
            }
        }

        for row in 0..self.row_count {
            match self.map.get(&Location { row: row, col: 0 }).unwrap().1 {
                EnclosedStatus::RightSide => return EnclosedStatus::RightSide,
                EnclosedStatus::LeftSide => return EnclosedStatus::LeftSide,
                _ => {}
            }

            match self
                .map
                .get(&Location {
                    row: row,
                    col: self.col_count - 1,
                })
                .unwrap()
                .1
            {
                EnclosedStatus::RightSide => return EnclosedStatus::RightSide,
                EnclosedStatus::LeftSide => return EnclosedStatus::LeftSide,
                _ => {}
            }
        }

        EnclosedStatus::Unknown
    }

    pub fn determine_inside_status(&mut self) -> EnclosedStatus {
        self.inside = Some(match self.find_outside_type() {
            EnclosedStatus::Unknown => EnclosedStatus::Unknown,
            EnclosedStatus::Loop => EnclosedStatus::Unknown,
            EnclosedStatus::RightSide => EnclosedStatus::LeftSide,
            EnclosedStatus::LeftSide => EnclosedStatus::RightSide,
        });

        self.inside.unwrap()
    }

    pub fn count_inside(&self) -> usize {
        self.map
            .values()
            .filter(|(_, status)| *status == self.inside.unwrap())
            .count()
    }
}

fn part1(text: &str) {
    let mut state = SolveState::from_input(text);
    while !state.step_both() {}

    println!("{}", state.steps);
}

fn part2(text: &str) {
    let mut state = SolveState::from_input(text);
    state.mark_loop();

    state.tag_loop_adjacent_cells();

    state.flood_sides();

    state.determine_inside_status();

    println!("{}", state.count_inside());
}
