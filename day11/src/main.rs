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

fn part1(text: &str) {
    let mut space = DMatrix::from_row_iterator(
        text.lines().count(),
        text.lines().next().unwrap().chars().count(),
        text.lines()
            .flat_map(|line| line.chars().map(|c| Space::from_char(&c))),
    );

    let cols_to_double: Vec<_> = space
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

    for n in cols_to_double.iter().rev() {
        space = space.insert_column(*n, Space::Empty);
    }

    let rows_to_double: Vec<_> = space
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

    for n in rows_to_double.iter().rev() {
        space = space.insert_row(*n, Space::Empty);
    }

    let mut galaxies: Vec<Point> = Vec::new();
    for (coln, col) in space.column_iter().enumerate() {
        for (rown, space) in col.iter().enumerate() {
            if *space == Space::Galaxy {
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

    println!("{}", sum);
}

fn part2(text: &str) {}
