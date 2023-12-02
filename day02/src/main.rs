use std::cmp::max;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Debug)]
struct Seen {
    red: i32,
    green: i32,
    blue: i32,
}

impl Seen {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn from_pull(pull: &str) -> Self {
        let mut out = Self::new();
        for dice in pull.split(", ") {
            let (count, color) = dice.split_once(" ").expect("no space in found");
            match color {
                "red" => out.red = count.parse().unwrap(),
                "green" => out.green = count.parse().unwrap(),
                "blue" => out.blue = count.parse().unwrap(),
                _ => panic!("Unexpected color {} in view {}", color, pull),
            };
        }
        out
    }

    fn update_max(&mut self, other: &Self) {
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);
        self.blue = max(self.blue, other.blue);
    }

    fn from_game(game: &str) -> Self {
        let mut out = Self::new();
        for pull in game.split("; ") {
            out.update_max(&Self::from_pull(pull));
        }
        out
    }

    fn possible(&self, bag: &Seen) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
}

const PART1_BAG: Seen = Seen {
    red: 12,
    green: 13,
    blue: 14,
};

fn part1(text: &str) {
    println!(
        "{}",
        text.lines()
            .filter_map(|line| {
                let (game_num, game) = line.split_once(": ").expect("no colon found");
                let seen = Seen::from_game(game);
                if seen.possible(&PART1_BAG) {
                    Some(game_num.split_once(" ").unwrap().1.parse::<i64>().unwrap())
                } else {
                    None
                }
            })
            .sum::<i64>()
    );
}

fn part2(_text: &str) {}
