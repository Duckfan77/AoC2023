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
enum Ground {
    Ash,
    Rock,
}

impl Ground {
    fn from_char(c: &char) -> Self {
        match c {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("Unexpected char in Ground::from_char: {c}"),
        }
    }
}

struct Pattern {
    pattern: DMatrix<Ground>,
}

impl Pattern {
    pub fn from_block(text: &str) -> Self {
        Self {
            pattern: DMatrix::from_row_iterator(
                text.lines().count(),
                text.lines().next().unwrap().chars().count(),
                text.lines()
                    .flat_map(|line| line.chars().map(|c| Ground::from_char(&c))),
            ),
        }
    }

    fn row_reflect(&self) -> usize {
        let rows = self.pattern.row_iter().collect::<Vec<_>>();
        for i in 1..self.pattern.nrows() {
            let left = &rows[0..i];
            let right = &rows[i..];
            if left.iter().rev().zip(right.iter()).all(|(l, r)| l == r) {
                return i;
            }
        }
        0
    }

    fn col_reflect(&self) -> usize {
        let cols = self.pattern.column_iter().collect::<Vec<_>>();
        for i in 1..self.pattern.ncols() {
            let left = &cols[0..i];
            let right = &cols[i..];
            if left.iter().rev().zip(right.iter()).all(|(l, r)| l == r) {
                return i;
            }
        }
        0
    }

    pub fn reflect(&self) -> usize {
        self.col_reflect() + 100 * self.row_reflect()
    }

    pub fn smudged_row_reflect(&self) -> usize {
        let rows = self.pattern.row_iter().collect::<Vec<_>>();
        for i in 1..self.pattern.nrows() {
            let left = &rows[0..i];
            let right = &rows[i..];
            if left.iter().rev().zip(right.iter()).fold(0, |acc, (l, r)| {
                l.iter().zip(r.iter()).filter(|(a, b)| a != b).count() + acc
            }) == 1
            {
                return i;
            }
        }
        0
    }

    pub fn smudged_col_reflect(&self) -> usize {
        let cols = self.pattern.column_iter().collect::<Vec<_>>();
        for i in 1..self.pattern.ncols() {
            let left = &cols[0..i];
            let right = &cols[i..];
            if left.iter().rev().zip(right.iter()).fold(0, |acc, (l, r)| {
                l.iter().zip(r.iter()).filter(|(a, b)| a != b).count() + acc
            }) == 1
            {
                return i;
            }
        }
        0
    }

    pub fn smudged_reflect(&self) -> usize {
        self.smudged_col_reflect() + 100 * self.smudged_row_reflect()
    }
}

fn part1(text: &str) {
    println!(
        "{}",
        text.split("\n\n")
            .map(|block| Pattern::from_block(block).reflect())
            .sum::<usize>()
    );
}

fn part2(text: &str) {
    println!(
        "{}",
        text.split("\n\n")
            .map(|block| Pattern::from_block(block).smudged_reflect())
            .sum::<usize>()
    );
}
