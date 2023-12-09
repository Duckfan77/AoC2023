use itertools::Itertools;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

struct Sequence {
    list: Vec<i64>,
    extrapolated: Option<i64>,
}

impl Sequence {
    fn from_line(line: &str) -> Self {
        Self {
            list: line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            extrapolated: None,
        }
    }

    fn from_iter(line: impl Iterator<Item = i64>) -> Self {
        Self {
            list: line.collect(),
            extrapolated: None,
        }
    }

    fn extrapolate(&mut self) -> i64 {
        if self.list.iter().all(|n| *n == 0) {
            self.extrapolated = Some(0);
            return 0;
        }

        let last = self.list.last().unwrap();
        let mut diff_seq =
            Sequence::from_iter(self.list.iter().tuple_windows().map(|(a, b)| b - a));
        let diff = diff_seq.extrapolate();

        self.extrapolated = Some(diff + last);
        self.extrapolated.unwrap()
    }
}

fn part1(text: &str) {
    println!(
        "{}",
        text.lines()
            .map(|line| Sequence::from_line(line).extrapolate())
            .sum::<i64>()
    );
}

fn part2(text: &str) {}
