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

fn convert_to_digits(text: String) -> String {
    // Conversions have the first and last letter preserved, to avoid issues where a
    // replacement removes a letter that was required for an earlier number. A
    // sufficiently evil input may be able to break this, but my input works.
    text.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn part2(text: &str) {
    part1(&convert_to_digits(text.into()))
}
