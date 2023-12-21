use std::{collections::HashMap, ops::Range, os::unix::process};

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    pub fn from_line(line: &str) -> Self {
        let line = line
            .replace("{", "")
            .replace("=", "")
            .replace("}", "")
            .replace("x", "")
            .replace("m", "")
            .replace("a", "")
            .replace("s", "");
        let mut segments = line.split(",");
        Self {
            x: segments.next().and_then(|s| s.parse().ok()).unwrap(),
            m: segments.next().and_then(|s| s.parse().ok()).unwrap(),
            a: segments.next().and_then(|s| s.parse().ok()).unwrap(),
            s: segments.next().and_then(|s| s.parse().ok()).unwrap(),
        }
    }

    pub fn get_var(self, var: &Var) -> i64 {
        match var {
            Var::X => self.x,
            Var::M => self.m,
            Var::A => self.a,
            Var::S => self.s,
        }
    }

    pub fn rating(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Op {
    Greater,
    Less,
}

enum Var {
    X,
    M,
    A,
    S,
}

impl Var {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unexpected string {s} in Var::from_char()"),
        }
    }
}

struct RuleSegment {
    op: Op,
    var: Var,
    value: i64,
    dest: String,
}

impl RuleSegment {
    fn from_text(text: &str) -> Self {
        let (test, dest) = text.split_once(":").unwrap();
        if let Some((var, val)) = test.split_once("<") {
            Self {
                op: Op::Less,
                var: Var::from_str(var),
                value: val.parse().unwrap(),
                dest: dest.to_string(),
            }
        } else if let Some((var, val)) = test.split_once(">") {
            Self {
                op: Op::Greater,
                var: Var::from_str(var),
                value: val.parse().unwrap(),
                dest: dest.to_string(),
            }
        } else {
            panic!("No Comparison in text {text}");
        }
    }

    fn run_rule(&self, part: &Part) -> Option<String> {
        match self.op {
            Op::Less => {
                if part.get_var(&self.var) < self.value {
                    Some(self.dest.clone())
                } else {
                    None
                }
            }
            Op::Greater => {
                if part.get_var(&self.var) > self.value {
                    Some(self.dest.clone())
                } else {
                    None
                }
            }
        }
    }
}

struct Rule {
    name: String,
    tests: Vec<RuleSegment>,
    default: String,
}

impl Rule {
    fn from_line(line: &str) -> (String, Self) {
        let (name, body) = line.split_once("{").unwrap();
        let (rules, default) = body.rsplit_once(",").unwrap();
        let tests = rules
            .split(",")
            .map(|r| RuleSegment::from_text(r))
            .collect();
        (
            name.to_string(),
            Self {
                name: name.to_string(),
                tests,
                default: default.replace("}", "").to_string(),
            },
        )
    }

    fn run_rule(&self, part: &Part) -> String {
        self.tests
            .iter()
            .find_map(|test| test.run_rule(part))
            .unwrap_or_else(|| self.default.clone())
    }
}

struct RuleSet {
    rules: HashMap<String, Rule>,
}

impl RuleSet {
    fn from_input(input: &str) -> Self {
        Self {
            rules: input.lines().map(|line| Rule::from_line(line)).collect(),
        }
    }

    fn process(&self, part: &Part) -> bool {
        let mut location = "in".to_string();
        while location != "A" && location != "R" {
            location = self.rules.get(&location).unwrap().run_rule(part);
        }

        location == "A"
    }

    fn scan_all(&self) -> i64 {
        let mut ranges = vec![(PartRange::new(), "in".to_string())];
        let mut passed = 0;

        while let Some((mut parts, key)) = ranges.pop() {
            let rule = self.rules.get(&key).unwrap();
            let mut need_default = true;
            for test in &rule.tests {
                let (pass, fail) = parts.split_with_rule(test);
                if let Some(pass) = pass {
                    if test.dest == "A" {
                        passed += pass.size();
                    } else if test.dest == "R" {
                        // rejected, do nothing
                    } else {
                        ranges.push((pass, test.dest.clone()))
                    };
                }

                if let Some(fail) = fail {
                    parts = fail;
                } else {
                    need_default = false;
                    break; // nothing more to keep chaining on
                }
            }
            if need_default {
                if rule.default == "A" {
                    passed += parts.size();
                } else if rule.default == "R" {
                    // rejected, do nothing
                } else {
                    ranges.push((parts, rule.default.clone()))
                };
            }
        }

        passed
    }
}

#[derive(Clone)]
struct PartRange {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn get_var(&self, var: &Var) -> &Range<i64> {
        match var {
            Var::X => &self.x,
            Var::M => &self.m,
            Var::A => &self.a,
            Var::S => &self.s,
        }
    }

    fn set_var(&self, var: &Var, range: Range<i64>) -> Self {
        let mut out = self.clone();
        match var {
            Var::X => out.x = range,
            Var::M => out.m = range,
            Var::A => out.a = range,
            Var::S => out.s = range,
        };
        out
    }

    fn split_with_rule(&self, rule: &RuleSegment) -> (Option<PartRange>, Option<PartRange>) {
        let mut split = rule.value;
        if rule.op == Op::Greater {
            split += 1
        }

        let (low, high) = self.get_var(&rule.var).split_at(split);

        if rule.op == Op::Less {
            (
                low.map(|range| self.set_var(&rule.var, range)),
                high.map(|range| self.set_var(&rule.var, range)),
            )
        } else {
            (
                high.map(|range| self.set_var(&rule.var, range)),
                low.map(|range| self.set_var(&rule.var, range)),
            )
        }
    }

    fn size(&self) -> i64 {
        let xsize = self.x.end - self.x.start;
        let msize = self.m.end - self.m.start;
        let asize = self.a.end - self.a.start;
        let ssize = self.s.end - self.s.start;
        let out = xsize * msize * asize * ssize;
        out
    }
}

trait Split {
    fn split_at(&self, i: i64) -> (Option<Range<i64>>, Option<Range<i64>>);
}

impl Split for Range<i64> {
    fn split_at(&self, i: i64) -> (Option<Range<i64>>, Option<Range<i64>>) {
        match (self.start < i, self.end > i) {
            (false, false) => (None, None), // something is horribly wrong, this is an invalid range
            (false, true) => (None, Some(self.clone())), // i is outside range
            (true, false) => (Some(self.clone()), None), // i is outside range
            (true, true) => (Some(self.start..i), Some(i..self.end)),
        }
    }
}

fn part1(text: &str) {
    let (rules, parts) = text.split_once("\n\n").unwrap();
    let rules = RuleSet::from_input(rules);
    println!(
        "{}",
        parts
            .lines()
            .map(|line| Part::from_line(line))
            .filter(|part| rules.process(part))
            .map(|part| part.rating())
            .sum::<i64>(),
    );
}

fn part2(text: &str) {
    let rules = RuleSet::from_input(text.split_once("\n\n").unwrap().0);
    println!("{}", rules.scan_all());
}
