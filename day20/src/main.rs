use std::collections::{HashMap, VecDeque};

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

type ModuleIdx = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PulseType {
    High,
    Low,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pulse {
    source: ModuleIdx,
    dest: ModuleIdx,
    pulse: PulseType,
}

impl Pulse {
    fn new(source: ModuleIdx, dest: ModuleIdx, pulse: PulseType) -> Self {
        Self {
            source,
            dest,
            pulse,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        source: HashMap<ModuleIdx, PulseType>,
    },
}

impl ModuleType {
    fn process_pulse(&mut self, pulse: &Pulse) -> Option<PulseType> {
        match self {
            ModuleType::Broadcast => Some(pulse.pulse),
            ModuleType::FlipFlop { on } => {
                if pulse.pulse == PulseType::Low {
                    *on = !*on;
                    if *on {
                        Some(PulseType::High)
                    } else {
                        Some(PulseType::Low)
                    }
                } else {
                    None // do nothing on High Pulse
                }
            }
            ModuleType::Conjunction { source } => {
                source.insert(pulse.source, pulse.pulse);
                if source.values().all(|pulse| *pulse == PulseType::High) {
                    Some(PulseType::Low)
                } else {
                    Some(PulseType::High)
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Module {
    index: ModuleIdx,
    mod_type: ModuleType,
    dests: Vec<ModuleIdx>,
}

impl Module {
    fn process_pulse(&mut self, pulse: &Pulse) -> VecDeque<Pulse> {
        if let Some(out_pulse) = self.mod_type.process_pulse(pulse) {
            self.dests
                .iter()
                .map(|dest| Pulse::new(self.index, *dest, out_pulse))
                .collect()
        } else {
            VecDeque::new()
        }
    }
}

struct ModuleSet {
    broadcast: ModuleIdx,
    modules: Vec<Module>,
    pulses: VecDeque<Pulse>,
    low_pulses_sent: u32,
    high_pulses_sent: u32,
    button_presses: u32,
}

impl ModuleSet {
    fn from_input(input: &str) -> Self {
        enum StrModType {
            Broadcast,
            Conjunction,
            FlipFlop,
        }

        impl StrModType {
            fn from_char(c: &char) -> Self {
                match c {
                    '%' => Self::FlipFlop,
                    '&' => Self::Conjunction,
                    'b' => Self::Broadcast,
                    _ => panic!("Unexpected character {c}"),
                }
            }
        }

        struct StrModule {
            name: String,
            dests: String,
            tpe: StrModType,
        }

        impl StrModule {
            fn from_line(line: &str) -> Self {
                let (name, dests) = line.split_once(" -> ").unwrap();
                let tpe = StrModType::from_char(&name.chars().next().unwrap());
                let name = name.trim_start_matches(['%', '&']).to_string();
                Self {
                    name,
                    dests: dests.to_string(),
                    tpe,
                }
            }
        }

        // Create intermediate maps from the input
        let str_modules: HashMap<String, StrModule> = input
            .lines()
            .map(|line| {
                let m = StrModule::from_line(line);
                (m.name.clone(), m)
            })
            .collect();

        let mut name_to_numbers: HashMap<String, ModuleIdx> = HashMap::new();
        let mut broadcast = 0;
        for name in str_modules.keys() {
            if name == "broadcaster" {
                broadcast = name_to_numbers.len();
            }
            name_to_numbers.insert(name.clone(), name_to_numbers.len());
        }

        // Construct real modules out of the maps
        let mut modules: Vec<_> = str_modules
            .iter()
            .map(|(name, strmod)| {
                let mod_type = match strmod.tpe {
                    StrModType::Broadcast => ModuleType::Broadcast,
                    StrModType::Conjunction => ModuleType::Conjunction {
                        source: HashMap::new(),
                    },
                    StrModType::FlipFlop => ModuleType::FlipFlop { on: false },
                };
                let dests: Vec<_> = strmod
                    .dests
                    .split(", ")
                    .map(|dst| *name_to_numbers.get(dst).unwrap_or(&usize::MAX))
                    .collect();
                Module {
                    index: *name_to_numbers.get(name).unwrap(),
                    mod_type,
                    dests,
                }
            })
            .collect();
        modules.sort_by_key(|module| module.index);

        // populate the sources for conjunction modules
        for i in 0..modules.len() {
            let sourcemod = modules[i].clone();
            for dst in sourcemod.dests {
                if dst != usize::MAX {
                    if let ModuleType::Conjunction { source } = &mut modules[dst].mod_type {
                        source.insert(sourcemod.index, PulseType::Low);
                    }
                }
            }
        }

        Self {
            broadcast,
            modules,
            pulses: VecDeque::new(),
            low_pulses_sent: 0,
            high_pulses_sent: 0,
            button_presses: 0,
        }
    }

    fn process_pulse(&mut self, pulse: &Pulse) -> bool {
        if pulse.dest == usize::MAX {
            if pulse.pulse == PulseType::Low {
                return true;
            } else {
                return false;
            }
        };
        self.pulses
            .append(&mut (self.modules[pulse.dest].process_pulse(&pulse)));

        return false;
    }

    fn press_button(&mut self) -> bool {
        self.button_presses += 1;
        self.pulses
            .push_back(Pulse::new(self.broadcast, self.broadcast, PulseType::Low));
        while let Some(pulse) = self.pulses.pop_front() {
            if pulse.pulse == PulseType::Low {
                self.low_pulses_sent += 1;
            } else {
                self.high_pulses_sent += 1;
            }
            if self.process_pulse(&pulse) == true {
                return true;
            }
        }
        return false;
    }

    fn pulse_product(&self) -> u32 {
        self.low_pulses_sent * self.high_pulses_sent
    }
}

fn part1(text: &str) {
    let mut modules = ModuleSet::from_input(text);
    for _ in 0..1000 {
        modules.press_button();
    }

    println!("{}", modules.pulse_product());
}

fn part2(text: &str) {
    let mut modules = ModuleSet::from_input(text);
    while !modules.press_button() {}

    println!("{}", modules.button_presses);
}
