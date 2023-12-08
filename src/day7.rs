use core::fmt;
use std::collections::{HashMap, HashSet};

use crate::Part;

pub fn run(lines: Vec<String>, part: Part) {
    let before = std::time::Instant::now();
    match part {
        Part::One => part1(lines),
        Part::Two => part2(lines),
    }
    println!("Elapsed: {:.2?}", before.elapsed());
}

fn part1(lines: Vec<String>) {
    let mut hands = lines
        .iter()
        .map(String::as_str)
        .map(Hand::from_str)
        .collect::<Vec<Hand>>();
    hands.sort();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |running_total, (idx, hand)| {
            println!("{} * {}", hand, idx + 1);
            running_total + ((idx as u32 + 1) * hand.bid)
        });
    println!("Total winnings: {total_winnings}");
}

fn part2(_lines: Vec<String>) {}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<RegularCard>,
    bid: u32,
    hand_type: Type,
}

impl Hand {
    fn from_str(line: &str) -> Self {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let cards = cards_str.chars().map(RegularCard::from_str).collect();
        let bid = str::parse::<u32>(bid_str).unwrap();
        let hand_type = Hand::hand_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn hand_type(cards: &Vec<RegularCard>) -> Type {
        let card_set = HashSet::<&RegularCard>::from_iter(cards.iter());
        match card_set.len() {
            1 => Type::FiveOfAKind,
            2 => {
                let first_card = &cards[0];
                let mut count = 0;
                for card in cards {
                    if *first_card == *card {
                        count += 1;
                    }
                }
                if count == 1 || count == 4 {
                    Type::FourOfAKind
                } else {
                    Type::FullHouse
                }
            }
            3 => {
                let mut counts = HashMap::<&RegularCard, u8>::new();
                for card in cards {
                    counts
                        .entry(card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                let mut hand_type = Type::ThreeOfAKind;
                for (_, count) in counts.iter() {
                    if *count == 2 {
                        hand_type = Type::TwoPair;
                        break;
                    }
                }
                hand_type
            }
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ord = self.hand_type.partial_cmp(&other.hand_type).unwrap();
        if ord.is_eq() {
            ord = self.cards.partial_cmp(&other.cards).unwrap();
        }
        Some(ord)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {:?}",
            self.cards.iter().map(|c| c.as_char()).collect::<String>(),
            self.bid,
            self.hand_type
        )
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum RegularCard {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl RegularCard {
    fn from_str(c: char) -> Self {
        match c {
            '2' => RegularCard::Two,
            '3' => RegularCard::Three,
            '4' => RegularCard::Four,
            '5' => RegularCard::Five,
            '6' => RegularCard::Six,
            '7' => RegularCard::Seven,
            '8' => RegularCard::Eight,
            '9' => RegularCard::Nine,
            'T' => RegularCard::Ten,
            'J' => RegularCard::Jack,
            'Q' => RegularCard::Queen,
            'K' => RegularCard::King,
            'A' => RegularCard::Ace,
            _ => panic!("Unknown card {c}"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            RegularCard::Two => '2',
            RegularCard::Three => '3',
            RegularCard::Four => '4',
            RegularCard::Five => '5',
            RegularCard::Six => '6',
            RegularCard::Seven => '7',
            RegularCard::Eight => '8',
            RegularCard::Nine => '9',
            RegularCard::Ten => 'T',
            RegularCard::Jack => 'J',
            RegularCard::Queen => 'Q',
            RegularCard::King => 'K',
            RegularCard::Ace => 'A',
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl JokerCard {
    fn from_str(c: char) -> Self {
        match c {
            'J' => JokerCard::Joker,
            '2' => JokerCard::Two,
            '3' => JokerCard::Three,
            '4' => JokerCard::Four,
            '5' => JokerCard::Five,
            '6' => JokerCard::Six,
            '7' => JokerCard::Seven,
            '8' => JokerCard::Eight,
            '9' => JokerCard::Nine,
            'T' => JokerCard::Ten,
            'Q' => JokerCard::Queen,
            'K' => JokerCard::King,
            'A' => JokerCard::Ace,
            _ => panic!("Unknown card {c}"),
        }
    }
    fn as_char(&self) -> char {
        match self {
            JokerCard::Joker => 'J',
            JokerCard::Two => '2',
            JokerCard::Three => '3',
            JokerCard::Four => '4',
            JokerCard::Five => '5',
            JokerCard::Six => '6',
            JokerCard::Seven => '7',
            JokerCard::Eight => '8',
            JokerCard::Nine => '9',
            JokerCard::Ten => 'T',
            JokerCard::Queen => 'Q',
            JokerCard::King => 'K',
            JokerCard::Ace => 'A',
        }
    }
}
