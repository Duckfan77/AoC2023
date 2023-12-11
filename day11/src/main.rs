use na::DMatrix;

extern crate nalgebra as na;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Space {
    Empty,
    Galaxy,
}

impl Space {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Unexpected char in from_char: {}", c),
        }
    }
}

struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn manhattan_dist(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn day11_core(text: &str, growth_factor: usize) -> usize {
    let space = DMatrix::from_row_iterator(
        text.lines().count(),
        text.lines().next().unwrap().chars().count(),
        text.lines()
            .flat_map(|line| line.chars().map(|c| Space::from_char(&c))),
    );

    let cols_to_grow: Vec<_> = space
        .column_iter()
        .enumerate()
        .filter_map(|(n, col)| {
            if col.iter().all(|s| *s == Space::Empty) {
                Some(n)
            } else {
                None
            }
        })
        .collect();

    let rows_to_grow: Vec<_> = space
        .row_iter()
        .enumerate()
        .filter_map(|(n, row)| {
            if row.iter().all(|s| *s == Space::Empty) {
                Some(n)
            } else {
                None
            }
        })
        .collect();

    let mut galaxies: Vec<Point> = Vec::new();
    for (rown, row) in space.row_iter().enumerate() {
        for (coln, space) in row.iter().enumerate() {
            if *space == Space::Galaxy {
                let rown = rown
                    + rows_to_grow
                        .iter()
                        .filter(|growth_row| rown > **growth_row)
                        .count()
                        * (growth_factor - 1);
                let coln = coln
                    + cols_to_grow
                        .iter()
                        .filter(|growth_col| coln > **growth_col)
                        .count()
                        * (growth_factor - 1);
                galaxies.push(Point::new(rown, coln));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += galaxies[i].manhattan_dist(&galaxies[j]);
        }
    }
    sum
}

fn part1(text: &str) {
    let sum = day11_core(text, 2);

    println!("{}", sum);
}

fn part2(text: &str) {
    let sum = day11_core(text, 1_000_000);

    println!("{}", sum);
}
