use std::collections::HashMap;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
enum HandType {
    // Discriminants added to force PartialOrd ordering
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn from_hand(hand: &Hand) -> Self {
        let mut card_map: HashMap<Card, i32> = HashMap::new();
        for c in hand.hand.iter() {
            *card_map.entry(*c).or_insert(0) += 1;
        }

        let mut card_counts = card_map.values().collect::<Vec<_>>();
        card_counts.sort();

        match (card_counts.len(), card_counts[card_counts.len() - 1]) {
            (1, 5) => Self::FiveKind,
            (2, 4) => Self::FourKind,
            (2, 3) => Self::FullHouse,
            (3, 3) => Self::ThreeKind,
            (3, 2) => Self::TwoPair,
            (4, 2) => Self::OnePair,
            (5, 1) => Self::HighCard,
            _ => {
                println!(
                    "Should be unreachable: {}, {}",
                    card_counts.len(),
                    card_counts[card_counts.len() - 1]
                );
                unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    C2 = 1,
    C3 = 2,
    C4 = 3,
    C5 = 4,
    C6 = 5,
    C7 = 6,
    C8 = 7,
    C9 = 8,
    CT = 9,
    CJ = 10,
    CQ = 11,
    CK = 12,
    CA = 13,
}

impl Card {
    fn from_char(c: &char) -> Self {
        match c {
            'A' => Self::CA,
            'K' => Self::CK,
            'Q' => Self::CQ,
            'J' => Self::CJ,
            'T' => Self::CT,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand: [Card; 5],
    bet: usize,
}

impl Hand {
    fn from_line(line: &str) -> Self {
        let (hand, bet) = line.split_once(' ').unwrap();
        Self {
            hand: hand
                .chars()
                .map(|c| Card::from_char(&c))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bet: bet.parse().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match HandType::from_hand(self).partial_cmp(&HandType::from_hand(&other)) {
            Some(core::cmp::Ordering::Equal) => self.hand.partial_cmp(&other.hand),
            ord => return ord,
        }
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match HandType::from_hand(self).partial_cmp(&HandType::from_hand(&other)) {
            Some(core::cmp::Ordering::Equal) => self.hand.partial_cmp(&other.hand).unwrap(),
            ord => return ord.unwrap(),
        }
    }
}

fn part1(text: &str) {
    let mut hands: Vec<_> = text.lines().map(|line| Hand::from_line(line)).collect();
    hands.sort();

    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bet)
            .sum::<usize>()
    );
}

fn part2(text: &str) {}
