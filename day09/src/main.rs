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
    extrapolated_forward: Option<i64>,
    extrapolated_backward: Option<i64>,
}

impl Sequence {
    fn from_line(line: &str) -> Self {
        Self {
            list: line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            extrapolated_forward: None,
            extrapolated_backward: None,
        }
    }

    fn from_iter(line: impl Iterator<Item = i64>) -> Self {
        Self {
            list: line.collect(),
            extrapolated_forward: None,
            extrapolated_backward: None,
        }
    }

    fn extrapolate_forward(&mut self) -> i64 {
        if self.list.iter().all(|n| *n == 0) {
            self.extrapolated_forward = Some(0);
            return 0;
        }

        let last = self.list.last().unwrap();
        let mut diff_seq =
            Sequence::from_iter(self.list.iter().tuple_windows().map(|(a, b)| b - a));
        let diff = diff_seq.extrapolate_forward();

        self.extrapolated_forward = Some(diff + last);
        self.extrapolated_forward.unwrap()
    }

    fn extrapolate_backward(&mut self) -> i64 {
        if self.list.iter().all(|n| *n == 0) {
            self.extrapolated_forward = Some(0);
            return 0;
        }

        let first = self.list.first().unwrap();
        let mut diff_seq =
            Sequence::from_iter(self.list.iter().tuple_windows().map(|(a, b)| b - a));
        let diff = diff_seq.extrapolate_backward();

        self.extrapolated_forward = Some(first - diff);
        self.extrapolated_forward.unwrap()
    }
}

fn part1(text: &str) {
    println!(
        "{}",
        text.lines()
            .map(|line| Sequence::from_line(line).extrapolate_forward())
            .sum::<i64>()
    );
}

fn part2(text: &str) {
    println!(
        "{}",
        text.lines()
            .map(|line| Sequence::from_line(line).extrapolate_backward())
            .sum::<i64>()
    );
}
