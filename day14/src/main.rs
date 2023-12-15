extern crate nalgebra as na;
use na::DMatrix;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

fn part1(text: &str) {
    let mut platform = Platform::from_input(text);

    platform.tilt_north();

    println!("{}", platform.calculate_load());
}

fn part2(text: &str) {}
