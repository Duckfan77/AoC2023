extern crate nalgebra as na;
use std::collections::HashMap;

use na::DMatrix;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    Round,
    Cube,
}

impl Square {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => panic!("Unexpected char in Square::from_char: {c}"),
        }
    }

    // This function is used in debugging, allow it to be deadcode
    #[allow(dead_code)]
    fn as_char(&self) -> char {
        match self {
            Square::Empty => '.',
            Square::Round => 'O',
            Square::Cube => '#',
        }
    }
}

struct Platform {
    map: DMatrix<Square>,
    past: HashMap<DMatrix<Square>, usize>,
}

impl Platform {
    fn from_input(input: &str) -> Self {
        Self {
            map: DMatrix::from_row_iterator(
                input.lines().count(),
                input.lines().next().unwrap().chars().count(),
                input
                    .lines()
                    .flat_map(|line| line.chars().map(|c| Square::from_char(&c))),
            ),
            past: HashMap::new(),
        }
    }

    fn tilt_north(&mut self) {
        let rows = self.map.nrows();
        for mut col in self.map.column_iter_mut() {
            let mut open = 0;
            for i in 0..rows {
                match col[i] {
                    Square::Empty => {}
                    Square::Round => {
                        if i > open {
                            // need to move the rock
                            col[open] = Square::Round;
                            col[i] = Square::Empty;
                        }
                        open = open + 1;
                    }
                    Square::Cube => {
                        // set open to next cell
                        open = i + 1
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let rows = self.map.nrows();
        for mut col in self.map.column_iter_mut() {
            let mut open: isize = rows as isize - 1;
            for i in (0..rows).rev() {
                match col[i] {
                    Square::Empty => {}
                    Square::Round => {
                        if i < open as usize {
                            // need to move the rock
                            col[open as usize] = Square::Round;
                            col[i] = Square::Empty;
                        }
                        open = open - 1;
                    }
                    Square::Cube => {
                        // set open to next cell
                        open = i as isize - 1
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let cols = self.map.ncols();
        for mut row in self.map.row_iter_mut() {
            let mut open = 0;
            for i in 0..cols {
                match row[i] {
                    Square::Empty => {}
                    Square::Round => {
                        if i > open {
                            // need to move the rock
                            row[open] = Square::Round;
                            row[i] = Square::Empty;
                        }
                        open = open + 1;
                    }
                    Square::Cube => {
                        // set open to next cell
                        open = i + 1
                    }
                }
            }
        }
    }

    fn tile_east(&mut self) {
        let cols = self.map.ncols();
        for mut row in self.map.row_iter_mut() {
            let mut open: isize = cols as isize - 1;
            for i in (0..cols).rev() {
                match row[i] {
                    Square::Empty => {}
                    Square::Round => {
                        if i < open as usize {
                            // need to move the rock
                            row[open as usize] = Square::Round;
                            row[i] = Square::Empty;
                        }
                        open = open - 1;
                    }
                    Square::Cube => {
                        // set open to next cell
                        open = i as isize - 1
                    }
                }
            }
        }
    }

    fn cycle(&mut self) -> bool {
        self.past.insert(self.map.clone(), self.past.len());
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tile_east();
        self.past.contains_key(&self.map)
    }

    fn calculate_load(&self) -> usize {
        let rows = self.map.nrows();
        self.map
            .row_iter()
            .enumerate()
            .map(|(row_n, row)| {
                //println!();
                row.iter()
                    .filter_map(|square| {
                        //print!("{}", square.as_char());
                        if *square == Square::Round {
                            Some(rows - row_n)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    // used for debug, don't warn for deadcode
    #[allow(dead_code)]
    fn print_map(&self) {
        for row in self.map.row_iter() {
            for square in row.iter() {
                print!("{}", square.as_char());
            }
            println!();
        }
    }
}

fn part1(text: &str) {
    let mut platform = Platform::from_input(text);

    platform.tilt_north();

    println!("{}", platform.calculate_load());
}

fn part2(text: &str) {
    let mut platform = Platform::from_input(text);

    for _ in 0..1_000_000_000 {
        if platform.cycle() {
            break;
        }
    }

    let cycle_len = platform.past.len() - platform.past.get(&platform.map).unwrap();
    let lead_length = platform.past.len() - cycle_len;

    let n = (0..lead_length)
        .chain(((lead_length)..(lead_length + cycle_len)).cycle())
        .nth(1_000_000_000)
        .unwrap();

    for (map, position) in platform.past.iter() {
        if *position == n {
            platform.map = map.clone();
            break;
        }
    }

    println!("{}", platform.calculate_load());
}
