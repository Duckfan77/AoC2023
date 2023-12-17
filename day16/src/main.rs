use std::{cmp::max, collections::HashMap};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BeamTip {
    location: Location,
    facing: Direction,
}

impl BeamTip {
    pub fn new(location: Location, facing: Direction) -> Self {
        Self { location, facing }
    }

    pub fn continue_in_line(&self) -> Self {
        Self {
            location: self.location.move_in_dir(&self.facing),
            facing: self.facing,
        }
    }

    pub fn move_in_dir(&self, dir: &Direction) -> Self {
        Self {
            location: self.location.move_in_dir(dir),
            facing: *dir,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ContraptionSegment {
    Empty,              // .
    HorizontalSplitter, // -
    VerticalSplitter,   // |
    DownMirror,         // \
    UpMirror,           // /
}

impl ContraptionSegment {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            '\\' => Self::DownMirror,
            '/' => Self::UpMirror,
            _ => panic!("Unexpected character {c} found in from_char"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Tile {
    object: ContraptionSegment,
    facing_up: bool,
    facing_down: bool,
    facing_left: bool,
    facing_right: bool,
}

impl Tile {
    fn new(object: ContraptionSegment) -> Self {
        Self {
            object,
            facing_up: false,
            facing_down: false,
            facing_left: false,
            facing_right: false,
        }
    }

    fn from_char(c: &char) -> Self {
        Self::new(ContraptionSegment::from_char(c))
    }

    fn passed_in_direction_mut(&mut self, dir: Direction) -> &mut bool {
        match dir {
            Direction::Up => &mut self.facing_up,
            Direction::Down => &mut self.facing_down,
            Direction::Left => &mut self.facing_left,
            Direction::Right => &mut self.facing_right,
        }
    }

    fn is_energized(&self) -> bool {
        self.facing_up || self.facing_down || self.facing_left || self.facing_right
    }

    fn handle_beam(&mut self, beam: &BeamTip) -> Vec<BeamTip> {
        // rename locally to reduce long lines that don't say much
        use ContraptionSegment as CS;
        use Direction as Dir;

        // Don't do anything if it's already been seen
        let is_duplicate = self.passed_in_direction_mut(beam.facing);
        if *is_duplicate {
            return Vec::new();
        }
        *is_duplicate = true;

        // Handle each case
        match self.object {
            CS::Empty => vec![beam.continue_in_line()], // continue in line
            CS::HorizontalSplitter => match beam.facing {
                Dir::Up | Dir::Down => {
                    vec![beam.move_in_dir(&Dir::Left), beam.move_in_dir(&Dir::Right)]
                } // split
                Dir::Left | Dir::Right => vec![beam.continue_in_line()], // continue in line
            },
            CS::VerticalSplitter => match beam.facing {
                Dir::Left | Dir::Right => {
                    vec![beam.move_in_dir(&Dir::Up), beam.move_in_dir(&Dir::Down)]
                } // split
                Dir::Up | Dir::Down => vec![beam.continue_in_line()], // continue in line
            },
            CS::DownMirror => match beam.facing {
                Dir::Up => vec![beam.move_in_dir(&Dir::Left)],
                Dir::Down => vec![beam.move_in_dir(&Dir::Right)],
                Dir::Left => vec![beam.move_in_dir(&Dir::Up)],
                Dir::Right => vec![beam.move_in_dir(&Dir::Down)],
            },
            CS::UpMirror => match beam.facing {
                Dir::Up => vec![beam.move_in_dir(&Dir::Right)],
                Dir::Down => vec![beam.move_in_dir(&Dir::Left)],
                Dir::Left => vec![beam.move_in_dir(&Dir::Down)],
                Dir::Right => vec![beam.move_in_dir(&Dir::Up)],
            },
        }
    }
}

struct ContraptionState {
    map: HashMap<Location, Tile>,
    beams: Vec<BeamTip>,
}

impl ContraptionState {
    fn from_input(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.char_indices().map(move |(col, c)| {
                        (
                            Location::new(row as isize, col as isize),
                            Tile::from_char(&c),
                        )
                    })
                })
                .collect(),
            beams: vec![BeamTip::new(Location::new(0, 0), Direction::Right)],
        }
    }

    fn set_starting_beam(&mut self, beam: BeamTip) {
        self.beams = vec![beam];
    }

    fn has_beams(&self) -> bool {
        !self.beams.is_empty()
    }

    fn run_single_beam(&mut self) {
        if let Some(beam) = self.beams.pop() {
            if let Some(tile) = self.map.get_mut(&beam.location) {
                self.beams.append(&mut tile.handle_beam(&beam))
            }
        }
    }

    fn run_beams(&mut self) {
        while self.has_beams() {
            self.run_single_beam()
        }
    }

    fn count_energized(&self) -> usize {
        self.map.values().filter(|tile| tile.is_energized()).count()
    }
}

fn part1(text: &str) {
    let mut contraption = ContraptionState::from_input(text);
    contraption.run_beams();
    println!("{}", contraption.count_energized());
}

fn part2(text: &str) {
    let nrows = text.lines().count() as isize;
    let ncols = text.lines().next().unwrap().chars().count() as isize;
    let mut best = 0;

    // top and bottom
    for col in 0..ncols {
        // top
        let mut contraption = ContraptionState::from_input(text);
        contraption.set_starting_beam(BeamTip::new(Location::new(0, col), Direction::Down));
        contraption.run_beams();
        best = max(best, contraption.count_energized());

        // bottom
        let mut contraption = ContraptionState::from_input(text);
        contraption.set_starting_beam(BeamTip::new(Location::new(ncols - 1, col), Direction::Up));
        contraption.run_beams();
        best = max(best, contraption.count_energized());
    }

    // left and right
    for row in 0..nrows {
        // left
        let mut contraption = ContraptionState::from_input(text);
        contraption.set_starting_beam(BeamTip::new(Location::new(row, 0), Direction::Right));
        contraption.run_beams();
        best = max(best, contraption.count_energized());

        // right
        let mut contraption = ContraptionState::from_input(text);
        contraption.set_starting_beam(BeamTip::new(Location::new(row, ncols - 1), Direction::Left));
        contraption.run_beams();
        best = max(best, contraption.count_energized());
    }

    println!("{}", best);
}
