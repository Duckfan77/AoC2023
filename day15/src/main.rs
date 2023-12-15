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

fn part1(text: &str) {
    println!(
        "{}",
        text.trim()
            .split(",")
            .map(|step| run_hash(step).0 as u32)
            .sum::<u32>()
    );
}

fn part2(text: &str) {}
