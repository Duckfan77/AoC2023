use std::collections::HashMap;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from_line(line: &str) -> Self {
        let (name, pair) = line.split_once(" = ").unwrap();
        let pair = pair.replace('(', "").replace(')', "");
        let (l, r) = pair.split_once(", ").unwrap();
        Self {
            name: name.to_string(),
            left: l.to_string(),
            right: r.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            // Yes, this is horrible and should be TryFrom, don't want to futz
            // about with errors for a toy task like this
            _ => panic!("converted char to direction with invalid arguments"),
        }
    }
}

struct Map {
    map: HashMap<String, Node>,
    state: String,
}

impl Map {
    fn from_body(body: &str) -> Self {
        Self {
            map: body
                .lines()
                .map(|line| Node::from_line(line))
                .map(|node| (node.name.clone(), node))
                .collect(),
            state: "AAA".to_string(),
        }
    }

    fn is_complete(&self) -> bool {
        self.state == "ZZZ"
    }

    fn step(&mut self, dir: &Direction) -> bool {
        self.state = match dir {
            Direction::Left => self.map.get(&self.state).unwrap().left.clone(),
            Direction::Right => self.map.get(&self.state).unwrap().right.clone(),
        };
        self.is_complete()
    }
}

fn part1(text: &str) {
    let (instructions, body) = text.split_once("\n\n").unwrap();
    let mut map = Map::from_body(body);
    let instructions: Vec<Direction> = instructions.chars().map(|c| c.into()).collect();

    let mut count = 0;
    for (step, dir) in instructions.iter().cycle().enumerate() {
        if map.step(dir) {
            count = step + 1;
            break;
        }
    }

    println!("{}", count);
}

fn part2(_text: &str) {}
