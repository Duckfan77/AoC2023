fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

fn part1(text: &str) {
    println!(
        "{}",
        text.lines()
            .map(|line| {
                line.chars()
                    .find(|c| c.is_digit(10))
                    .expect("No digits found")
                    .to_digit(10)
                    .unwrap()
                    * 10
                    + line
                        .chars()
                        .rfind(|c| c.is_digit(10))
                        .expect("No digits found")
                        .to_digit(10)
                        .unwrap()
            })
            .sum::<u32>()
    );
}

fn part2(text: &str) {}
