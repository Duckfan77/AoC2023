use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Plot {
    Rock,
    Garden,
}

impl Plot {
    fn from_char(c: &char) -> Self {
        match c {
            '.' | 'S' => Self::Garden,
            '#' => Self::Rock,
            _ => panic!("Unexpected char {c} in Plot::from_char()"),
        }
    }
}

struct Map {
    steps_taken: usize,
    plots: HashMap<Location, Plot>,
    current_locations: HashSet<Location>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let mut start = Location::new(0, 0);
        let mut plots = HashMap::new();
        for (rown, line) in input.lines().enumerate() {
            for (coln, c) in line.char_indices() {
                if c == 'S' {
                    start = Location::new(rown as isize, coln as isize);
                }
                plots.insert(
                    Location::new(rown as isize, coln as isize),
                    Plot::from_char(&c),
                );
            }
        }

        let mut locations = HashSet::new();
        locations.insert(start);

        Self {
            steps_taken: 0,
            plots: plots,
            current_locations: locations,
        }
    }

    fn is_valid(&self, loc: &Location) -> bool {
        match self.plots.get(loc) {
            Some(Plot::Garden) => true,
            Some(Plot::Rock) | None => false,
        }
    }

    fn step(&mut self) {
        self.steps_taken += 1;
        self.current_locations = self
            .current_locations
            .iter()
            .flat_map(|loc| loc.adjacents().filter(|new_loc| self.is_valid(&new_loc)))
            .collect();
    }

    fn run_steps(&mut self, step_count: usize) {
        for _ in 0..step_count {
            self.step();
        }
    }

    fn count_reachable(&self) -> usize {
        self.current_locations.len()
    }
}

const PART1_STEPS_TO_TAKE: usize = 64;

fn part1(text: &str) {
    let mut map = Map::from_input(text);
    map.run_steps(PART1_STEPS_TO_TAKE);
    println!("{}", map.count_reachable());
}

fn part2(text: &str) {}
