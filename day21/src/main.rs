use rayon::prelude::*;
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

    fn mod_sizes(&self, rowcap: isize, colcap: isize) -> Self {
        Self {
            row: self.row.rem_euclid(rowcap),
            col: self.col.rem_euclid(colcap),
        }
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
    new_locations: HashSet<Location>,
    rown: isize,
    coln: isize,
    odd_step_reached: HashSet<Location>,
    even_step_reached: HashSet<Location>,
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
            new_locations: locations.clone(),
            rown: input.lines().count() as isize,
            coln: input.lines().next().unwrap().chars().count() as isize,
            odd_step_reached: HashSet::new(),
            even_step_reached: locations,
        }
    }

    fn is_valid(&self, loc: &Location) -> bool {
        match self.plots.get(loc) {
            Some(Plot::Garden) => true,
            Some(Plot::Rock) | None => false,
        }
    }

    fn is_valid_infinite(&self, loc: &Location) -> bool {
        self.is_valid(&loc.mod_sizes(self.rown, self.coln))
    }

    fn step(&mut self) {
        self.steps_taken += 1;

        let mut reachable: HashSet<Location> = self
            .new_locations
            .par_iter()
            .flat_map_iter(|loc| loc.adjacents().filter(|new_loc| self.is_valid(&new_loc)))
            .collect();

        let step_set = if self.steps_taken % 2 == 0 {
            &mut self.even_step_reached
        } else {
            &mut self.odd_step_reached
        };

        reachable.retain(|loc| step_set.insert(*loc));

        self.new_locations = reachable;
    }

    fn run_steps(&mut self, step_count: usize) {
        for _ in 0..step_count {
            self.step();
        }
    }

    fn step_infinite(&mut self) {
        self.steps_taken += 1;
        let mut reachable: HashSet<Location> = self
            .new_locations
            .par_iter()
            .flat_map_iter(|loc| {
                loc.adjacents()
                    .filter(|new_loc| self.is_valid_infinite(&new_loc))
            })
            .collect();

        let step_set = if self.steps_taken % 2 == 0 {
            &mut self.even_step_reached
        } else {
            &mut self.odd_step_reached
        };

        reachable.retain(|loc| step_set.insert(*loc));

        self.new_locations = reachable;
    }

    fn run_steps_infinite(&mut self, step_count: usize) {
        for _ in 0..step_count {
            self.step_infinite();
        }
    }

    fn count_reachable(&self) -> usize {
        if self.steps_taken % 2 == 0 {
            self.even_step_reached.len()
        } else {
            self.odd_step_reached.len()
        }
    }
}

const PART1_STEPS_TO_TAKE: usize = 64;

fn part1(text: &str) {
    let mut map = Map::from_input(text);
    map.run_steps(PART1_STEPS_TO_TAKE);
    println!("{}", map.count_reachable());
}

const PART2_STEPS_TO_TAKE: usize = 2_6501_365;

fn part2(text: &str) {
    let mut map = Map::from_input(text);
    map.run_steps_infinite(PART2_STEPS_TO_TAKE);
    println!("{}", map.count_reachable());
}
