fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

struct Race {
    time: i64,
    dist: i64,
}

impl Race {
    fn new(time: i64, dist: i64) -> Self {
        Self { time, dist }
    }

    fn ways_to_beat(&self) -> usize {
        (0..self.time)
            .into_iter()
            .filter(|hold_time| ((self.time - hold_time) * hold_time) > self.dist)
            .count()
    }
}

fn part1(text: &str) {
    let mut lines = text.lines();
    println!(
        "{}",
        lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|t| t.parse::<i64>().unwrap())
            .zip(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .skip(1)
                    .map(|d| d.parse::<i64>().unwrap()),
            )
            .map(|(time, dist)| Race::new(time, dist).ways_to_beat())
            .product::<usize>()
    );
}

fn part2(text: &str) {}
