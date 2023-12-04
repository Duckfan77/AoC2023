use std::collections::HashSet;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

struct Card {
    winning: HashSet<i32>,
    have: HashSet<i32>,
}

impl Card {
    pub fn from_line(line: &str) -> Self {
        let (win, have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        Self {
            winning: win
                .trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect(),
            have: have
                .trim()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect(),
        }
    }

    fn overlap_to_points(overlap: usize) -> i32 {
        if overlap == 0 {
            0
        } else {
            2_i32.pow((overlap - 1) as u32)
        }
    }

    pub fn score(&self) -> i32 {
        Self::overlap_to_points(self.winning.intersection(&self.have).count())
    }
}

fn part1(text: &str) {
    println!(
        "{}",
        text.lines()
            .map(|card| Card::from_line(card).score())
            .sum::<i32>()
    );
}

fn part2(text: &str) {}
