use std::{
    cmp::{max, min},
    collections::HashMap,
    str::FromStr,
    string::ParseError,
};

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Dug {
    Trench(String),
    Start,
    Fill,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Location {
    row: isize,
    col: isize,
}

impl Location {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn cells_in_dir(&self, dir: Direction, count: usize) -> impl Iterator<Item = Self> + '_ {
        (1..=count as isize).map(move |m| match dir {
            Direction::Up => Location::new(self.row + m, self.col),
            Direction::Down => Location::new(self.row - m, self.col),
            Direction::Left => Location::new(self.row, self.col - m),
            Direction::Right => Location::new(self.row, self.col + m),
        })
    }

    fn cell_in_dir(&self, dir: Direction, count: usize) -> Location {
        let count = count as isize;
        match dir {
            Direction::Up => Location::new(self.row + count, self.col),
            Direction::Down => Location::new(self.row - count, self.col),
            Direction::Left => Location::new(self.row, self.col - count),
            Direction::Right => Location::new(self.row, self.col + count),
        }
    }

    fn move_in_dir(&self, dir: Direction) -> Location {
        self.cell_in_dir(dir, 1)
    }

    fn adjacents(&self) -> impl Iterator<Item = Self> + '_ {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|dir| self.move_in_dir(*dir))
    }
}

impl From<(isize, isize)> for Location {
    fn from((row, col): (isize, isize)) -> Self {
        Self { row, col }
    }
}

impl Into<(isize, isize)> for Location {
    fn into(self) -> (isize, isize) {
        (self.row, self.col)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: &char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid char {c} in Direction::from_char()"),
        }
    }
}

struct Step {
    dir: Direction,
    distance: usize,
    color: String,
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let dir = Direction::from_char(&iter.next().and_then(|s| s.chars().next()).unwrap());
        let dist = iter.next().and_then(|d| d.parse().ok()).unwrap();
        let color = iter
            .next()
            .unwrap()
            .replace("(", "")
            .replace("#", "")
            .replace(")", "");
        Ok(Self {
            dir,
            distance: dist,
            color,
        })
    }
}

struct Map {
    map: HashMap<Location, Dug>,
    location: Location,
    uppermost: isize,
    lowermost: isize,
    leftmost: isize,
}

impl Map {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Location::new(0, 0), Dug::Start);
        Self {
            map,
            location: Location::new(0, 0),
            uppermost: 0,
            lowermost: 0,
            leftmost: 0,
        }
    }

    fn run_step(&mut self, step: &Step) {
        let trench = Dug::Trench(step.color.clone());
        for loc in self.location.cells_in_dir(step.dir, step.distance) {
            self.map.insert(loc, trench.clone());
        }
        self.location = self.location.cell_in_dir(step.dir, step.distance);
        self.uppermost = max(self.uppermost, self.location.row);
        self.lowermost = min(self.lowermost, self.location.row);
        self.leftmost = min(self.leftmost, self.location.col);
    }

    fn run_input(&mut self, input: &str) {
        for step in input.lines().map(|line| line.parse().unwrap()) {
            self.run_step(&step);
        }
    }

    fn find_interior(&mut self) -> Location {
        let mut scan = Location::new((self.uppermost + self.lowermost) / 2, self.leftmost);
        while let None = self.map.get(&scan) {
            scan.col += 1;
        }

        // Found the trench, move one more to get to the internals
        scan.col += 1;
        scan
    }

    fn flood_interior(&mut self) {
        let mut unprocessed = vec![self.find_interior()];
        while let Some(cell) = unprocessed.pop() {
            if let None = self.map.insert(cell, Dug::Fill) {
                unprocessed.extend(cell.adjacents().filter(|loc| self.map.get(loc) == None));
            }
        }
    }

    fn count_pool(&self) -> usize {
        self.map.len()
    }
}

fn part1(text: &str) {
    let mut pool = Map::new();
    pool.run_input(text);
    pool.flood_interior();
    println!("{}", pool.count_pool());
}

fn part2(_text: &str) {}
