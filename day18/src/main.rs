use std::{str::FromStr, string::ParseError};

use itertools::Itertools;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
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

    fn cell_in_dir(&self, dir: Direction, count: usize) -> Location {
        let count = count as isize;
        match dir {
            Direction::Up => Location::new(self.row + count, self.col),
            Direction::Down => Location::new(self.row - count, self.col),
            Direction::Left => Location::new(self.row, self.col - count),
            Direction::Right => Location::new(self.row, self.col + count),
        }
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

impl Step {
    fn from_color(&self) -> Step {
        let mut color_chars = self.color.chars();
        let mut dist = 0;
        for _ in 0..5 {
            dist = dist * 16 + color_chars.next().and_then(|c| c.to_digit(16)).unwrap();
        }
        let dir = match color_chars.next().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Right,
            _ => panic!("Unexpected char in from_color"),
        };
        assert!(color_chars.next() == None); // should have finished it

        Step {
            dir,
            distance: dist as usize,
            color: "".to_string(),
        }
    }
}

struct VertexMap {
    vertices: Vec<Location>,
    location: Location,
}

fn normalize(n: isize) -> isize {
    if n == 0 {
        0
    } else {
        n / n.abs()
    }
}

impl VertexMap {
    fn new() -> Self {
        Self {
            vertices: vec![Location::new(0, 0)],
            location: Location::new(0, 0),
        }
    }

    fn run_step(&mut self, step: &Step) {
        let vertex = self.location.cell_in_dir(step.dir, step.distance);
        self.vertices.push(vertex);
        self.location = vertex;
    }

    fn run_step_using_color(&mut self, step: &Step) {
        self.run_step(&step.from_color());
    }

    fn run_input(&mut self, input: &str) {
        for step in input.lines().map(|line| line.parse().unwrap()) {
            self.run_step(&step);
        }
        self.vertices.pop(); //last vertex is adjacent to the first one, just complicates things
    }

    fn run_input_using_color(&mut self, input: &str) {
        for step in input.lines().map(|line| line.parse().unwrap()) {
            self.run_step_using_color(&step);
        }
    }

    fn calculate_area(&self) -> f64 {
        let true_vertices = self
            .vertices
            .iter()
            .circular_tuple_windows()
            .map(|(prev, current, next)| {
                let base_row = current.row as f64;
                let base_col = current.col as f64;

                let rdiff1 = normalize(current.row - prev.row);
                let cdiff1 = normalize(current.col - prev.col);
                let rdiff2 = normalize(next.row - current.row);
                let cdiff2 = normalize(next.col - current.col);

                //println!("({base_row}, {base_col}): {rdiff1}, {cdiff1}, {rdiff2}, {cdiff2}");

                match (rdiff1, cdiff1, rdiff2, cdiff2) {
                    (0, 1, 1, 0) => (base_row - 0.5, base_col + 0.5),
                    (0, -1, 1, 0) => (base_row - 0.5, base_col - 0.5),
                    (0, 1, -1, 0) => (base_row + 0.5, base_col + 0.5),
                    (0, -1, -1, 0) => (base_row + 0.5, base_col - 0.5),
                    (1, 0, 0, 1) => (base_row + 0.5, base_col - 0.5),
                    (-1, 0, 0, 1) => (base_row - 0.5, base_col - 0.5),
                    (1, 0, 0, -1) => (base_row + 0.5, base_col + 0.5),
                    (-1, 0, 0, -1) => (base_row - 0.5, base_col + 0.5),
                    _ => panic!("Invalid differences: {rdiff1}, {cdiff1}, {rdiff2}, {cdiff2}"),
                }
            })
            .collect::<Vec<_>>();

        //println!("{:?}", true_vertices);
        /*println!(
            "{:?}",
            self.vertices
                .iter()
                .map(|loc| (*loc).into())
                .collect::<Vec<(isize, isize)>>()
        );*/

        true_vertices
            .iter()
            .circular_tuple_windows()
            .fold(0.0, |acc, (current, prev)| {
                acc + ((prev.0 + current.0) * (prev.1 - current.1))
            })
            / 2.0
    }
}

fn part1(text: &str) {
    let mut pool = VertexMap::new();
    pool.run_input(text);
    println!("{}", pool.calculate_area());
}

fn part2(text: &str) {
    let mut pool = VertexMap::new();
    pool.run_input_using_color(text);
    println!("{}", pool.calculate_area());
}
