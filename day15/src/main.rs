use std::num::Wrapping;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

fn run_hash(step: &str) -> Wrapping<u8> {
    step.bytes()
        .map(|n| Wrapping(n))
        .fold(Wrapping(0), |current, new| (current + new) * Wrapping(17))
}

struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Default)]
struct Box {
    number: u32,
    contents: Vec<Lens>,
}

impl Box {
    fn new(number: u32) -> Self {
        Self {
            number,
            contents: Vec::new(),
        }
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(index) = self.contents.iter().position(|lens| lens.label == label) {
            self.contents.remove(index);
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        if let Some(index) = self
            .contents
            .iter()
            .position(|contents| contents.label == lens.label)
        {
            self.contents[index] = lens;
        } else {
            self.contents.push(lens);
        }
    }

    fn calculate_power(&self) -> u32 {
        self.contents
            .iter()
            .enumerate()
            .map(|(slot, lens)| lens.focal_length * (slot as u32 + 1) * (self.number + 1))
            .sum()
    }
}

struct BoxSet {
    boxes: Vec<Box>,
}

impl BoxSet {
    fn new() -> Self {
        Self {
            boxes: (0..256).map(|i| Box::new(i)).collect(),
        }
    }

    fn run_rule(&mut self, rule: &str) {
        if let Some((label, focus)) = rule.split_once("=") {
            let lens = Lens {
                label: label.to_string(),
                focal_length: focus.parse().unwrap(),
            };
            let index = run_hash(label).0 as usize;
            self.boxes[index].add_lens(lens);
        } else if let Some((label, _)) = rule.split_once("-") {
            let index = run_hash(label).0 as usize;
            self.boxes[index].remove_lens(label);
        } else {
            unreachable!()
        }
    }

    fn calculate_power(&self) -> u32 {
        self.boxes.iter().map(|b| b.calculate_power()).sum()
    }
}
fn part1(text: &str) {
    println!(
        "{}",
        text.trim()
            .split(",")
            .map(|step| run_hash(step).0 as u32)
            .sum::<u32>()
    );
}

fn part2(text: &str) {
    let mut boxes = BoxSet::new();
    for step in text.trim().split(",") {
        boxes.run_rule(step);
    }

    println!("{}", boxes.calculate_power());
}
