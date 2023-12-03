use std::collections::{HashMap, HashSet};

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

type NumberIndex = usize;

#[derive(Copy, Debug, Clone)]
struct Number {
    number: i32,
    counted: bool,
}

impl Number {
    fn new(number: i32) -> Self {
        Self {
            number,
            counted: false,
        }
    }
}

enum Cell {
    Symbol(char),
    NumberCell(NumberIndex),
}

impl Cell {
    fn is_symbol(&self) -> bool {
        match self {
            Cell::Symbol(_) => true,
            _ => false,
        }
    }
}

struct Schematic {
    grid: HashMap<(isize, isize), Cell>,
    numbers: Vec<Number>,
}

impl Schematic {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            numbers: Vec::new(),
        }
    }

    fn is_symbol(c: &char) -> bool {
        !(c.is_digit(10) || *c == '.')
    }

    fn from_str(text: &str) -> Self {
        let mut out = Self::new();

        for (row_n, row) in text.lines().enumerate() {
            let mut number = 0;
            let mut number_start: Option<usize> = None;
            for (col_n, c) in row.chars().enumerate() {
                match (number_start, c.to_digit(10)) {
                    (Some(start), Some(digit)) => {
                        number *= 10;
                        number += digit as i32;

                        if col_n == row.len() - 1 {
                            // special case, this is the end of the row
                            for position in start..=col_n {
                                out.grid.insert(
                                    (row_n as isize, position as isize),
                                    Cell::NumberCell(out.numbers.len()),
                                );
                            }
                            out.numbers.push(Number::new(number));
                            number = 0;
                            number_start = None;
                        }
                    }

                    (None, Some(digit)) => {
                        number_start = Some(col_n);
                        number = digit as i32;

                        if col_n == row.len() - 1 {
                            // special case, this is the end of the row, and it's a 1 digit number
                            out.grid.insert(
                                (row_n as isize, col_n as isize),
                                Cell::NumberCell(out.numbers.len()),
                            );
                            out.numbers.push(Number::new(number));
                            number = 0;
                            number_start = None;
                        }
                    }

                    (Some(start), None) => {
                        for position in start..col_n {
                            out.grid.insert(
                                (row_n as isize, position as isize),
                                Cell::NumberCell(out.numbers.len()),
                            );
                        }
                        out.numbers.push(Number::new(number));
                        number = 0;
                        number_start = None;
                    }

                    (None, None) => { /* No behavior required in this case */ }
                };
                if Self::is_symbol(&c) {
                    out.grid
                        .insert((row_n as isize, col_n as isize), Cell::Symbol(c));
                }
            }
        }

        out
    }

    fn engine_sum(&mut self) -> i32 {
        let mut sum = 0;

        for ((row_n, col_n), _) in self.grid.iter().filter(|((_, _), cell)| cell.is_symbol()) {
            for row_offset in -1..=1 {
                for col_offset in -1..=1 {
                    if let Some(Cell::NumberCell(index)) =
                        self.grid.get(&(row_n + row_offset, col_n + col_offset))
                    {
                        let number = self.numbers.get_mut(*index).expect("Index out of bounds");
                        if !number.counted {
                            sum += number.number;
                            number.counted = true;
                        }
                    }
                }
            }
        }

        sum
    }

    fn gear_product_sum(&self) -> i32 {
        let mut sum = 0;

        for ((row_n, col_n), _) in self.grid.iter().filter(|((_, _), cell)| {
            if let Cell::Symbol('*') = cell {
                true
            } else {
                false
            }
        }) {
            let mut indices = HashSet::new();
            for row_offset in -1..=1 {
                for col_offset in -1..=1 {
                    if let Some(Cell::NumberCell(index)) =
                        self.grid.get(&(row_n + row_offset, col_n + col_offset))
                    {
                        indices.insert(*index);
                    }
                }
            }
            if indices.len() == 2 {
                sum += indices
                    .iter()
                    .map(|i| self.numbers[*i].number)
                    .product::<i32>();
            }
        }

        sum
    }
}

fn part1(text: &str) {
    let mut s = Schematic::from_str(text);
    println!("{}", s.engine_sum());
}

fn part2(text: &str) {
    let s = Schematic::from_str(text);
    println!("{}", s.gear_product_sum());
}
