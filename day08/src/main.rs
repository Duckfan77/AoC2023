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

#[derive(Debug, Clone)]
struct IntNode {
    index: usize,
    is_end: bool,
    left: usize,
    right: usize,
}

struct MapSet {
    map: Vec<IntNode>,
    states: Vec<usize>,
}

impl MapSet {
    fn from_body(body: &str) -> Self {
        let string_map: HashMap<String, Node> = body
            .lines()
            .map(|line| Node::from_line(line))
            .map(|node| (node.name.clone(), node))
            .collect();
        let name_map: HashMap<String, usize> = string_map
            .keys()
            .cloned()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();
        let mut map: Vec<_> = name_map
            .iter()
            .map(|(name, index)| {
                let str_node = string_map.get(name).unwrap();
                IntNode {
                    index: *index,
                    is_end: name.ends_with('Z'),
                    left: *name_map.get(&str_node.left).unwrap(),
                    right: *name_map.get(&str_node.right).unwrap(),
                }
            })
            .collect();
        map.sort_by_key(|node| node.index);

        let states = name_map
            .iter()
            .filter_map(|(name, index)| {
                if name.ends_with('A') {
                    Some(*index)
                } else {
                    None
                }
            })
            .collect();

        //println!("{:?}\n{:?}\n{:?}\n", name_map, map, states);

        Self { map, states }
    }

    fn is_complete(&self) -> bool {
        self.states.iter().all(|index| self.map[*index].is_end)
    }

    fn step_all(&mut self, dir: &Direction) -> bool {
        //println!("{:?}", self.states);
        for state in self.states.iter_mut() {
            *state = match dir {
                Direction::Left => self.map[*state].left,
                Direction::Right => self.map[*state].right,
            }
        }

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

fn part2(text: &str) {
    let (instructions, body) = text.split_once("\n\n").unwrap();
    let mut maps = MapSet::from_body(body);
    let instructions: Vec<Direction> = instructions.chars().map(|c| c.into()).collect();

    let mut count = 0;
    for (step, dir) in instructions.iter().cycle().enumerate() {
        if maps.step_all(dir) {
            count = step + 1;
            break;
        }
    }

    println!("{}", count);
}
