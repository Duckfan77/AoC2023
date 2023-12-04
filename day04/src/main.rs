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

    pub fn overlap(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }

    pub fn score(&self) -> i32 {
        Self::overlap_to_points(self.overlap())
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

fn part2(text: &str) {
    let mut total = 0;
    let mut cards: Vec<(i32, Card)> = text
        .lines()
        .map(|card| (1, Card::from_line(card)))
        .collect();
    for i in 0..cards.len() {
        let current_count = cards[i].0;
        total += current_count;
        for add in (i + 1)..=(i + cards[i].1.overlap()) {
            let (mod_count, _) = cards.get_mut(add).unwrap();
            *mod_count += current_count;
        }
    }
    println!("{}", total);
}
