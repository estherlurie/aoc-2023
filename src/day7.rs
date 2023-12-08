use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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
    compute_winnings::<RegularCard>(lines);
}

fn part2(lines: Vec<String>) {
    compute_winnings::<JokerCard>(lines);
}

fn compute_winnings<C: Card + Eq + Hash + Ord + PartialEq + PartialOrd>(lines: Vec<String>) {
    let mut hands = lines
        .iter()
        .map(String::as_str)
        .map(Hand::<C>::from_str)
        .collect::<Vec<Hand<C>>>();
    hands.sort();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |running_total, (idx, hand)| {
            running_total + ((idx as u32 + 1) * hand.bid)
        });
    println!("Total winnings: {total_winnings}");
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand<C>
where
    C: Card,
{
    hand_type: Type,
    cards: Vec<C>,
    bid: u32,
}

impl<C: Card + Eq + Hash + PartialEq> Hand<C> {
    fn from_str(line: &str) -> Self {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();
        let cards = cards_str.chars().map(Card::from_str).collect::<Vec<C>>();
        let bid = str::parse::<u32>(bid_str).unwrap();
        let hand_type = Hand::hand_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn hand_type(cards: &[C]) -> Type {
        C::hand_type(cards)
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

trait Card {
    fn from_str(c: char) -> Self
    where
        Self: Sized;

    fn hand_type(cards: &[Self]) -> Type
    where
        Self: Sized;
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum RegularCard {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card for RegularCard {
    fn from_str(c: char) -> Self {
        match c {
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                RegularCard::Num(c.to_digit(10).unwrap())
            }
            'T' => RegularCard::Num(10),
            'J' => RegularCard::Jack,
            'Q' => RegularCard::Queen,
            'K' => RegularCard::King,
            'A' => RegularCard::Ace,
            _ => panic!("Unknown card {c}"),
        }
    }

    fn hand_type(cards: &[RegularCard]) -> Type {
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

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum JokerCard {
    Joker,
    Num(u32),
    Queen,
    King,
    Ace,
}

impl JokerCard {
    fn as_regular_card(&self) -> RegularCard {
        match self {
            JokerCard::Num(i) => RegularCard::Num(*i),
            JokerCard::Joker => RegularCard::Jack,
            JokerCard::Queen => RegularCard::Queen,
            JokerCard::King => RegularCard::King,
            JokerCard::Ace => RegularCard::Ace,
        }
    }
}

impl Card for JokerCard {
    fn from_str(c: char) -> Self {
        match c {
            'J' => JokerCard::Joker,
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                JokerCard::Num(c.to_digit(10).unwrap())
            }
            'T' => JokerCard::Num(10),
            'Q' => JokerCard::Queen,
            'K' => JokerCard::King,
            'A' => JokerCard::Ace,
            _ => panic!("Unknown card {c}"),
        }
    }

    fn hand_type(cards: &[JokerCard]) -> Type {
        // Initialize count map
        let mut counts = HashMap::<&JokerCard, u8>::new();
        for c in cards {
            counts.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        // Get joker count
        let joker_count = counts.get(&JokerCard::Joker).unwrap_or(&0);

        /*
        ZYXWV - No Joker
        ZYXWJ - 1 Joker, all diff -> 1 pair
        ZZYXJ - 1 Joker, 1 pair -> 3 of a kind
        ZZYYJ - 1 Joker, 2 pair -> full house
        ZZZYJ - 1 Joker, 3 of a kind -> 4 of a kind
        ZZZZJ - 1 Joker, 4 of a kind -> 5 of a kind
        ZYXJJ - 2 Joker, 1 pair -> 3 of a kind
        ZZYJJ - 2 Joker, 2 pair -> 4 of a kind
        ZZZJJ - 2 Joker, full house -> 5 of a kind
        ZYJJJ - 3 Joker, 3 of a kind -> 4 of a kind
        ZZJJJ - 3 Joker, full house -> 5 of a kind
        ZJJJJ - 4 Joker, 4 of a kind -> 5 of a kind
        JJJJJ - 5 Joker, 5 of a kind
        */
        let regular_hand_type = RegularCard::hand_type(
            &cards
                .iter()
                .map(|c| c.as_regular_card())
                .collect::<Vec<RegularCard>>(),
        );
        match (joker_count, regular_hand_type) {
            (0, regular_hand_type) => regular_hand_type,
            (1, Type::HighCard) => Type::OnePair,
            (1, Type::OnePair) | (2, Type::OnePair) => Type::ThreeOfAKind,
            (1, Type::TwoPair) => Type::FullHouse,
            (1, Type::ThreeOfAKind) | (2, Type::TwoPair) | (3, Type::ThreeOfAKind) => {
                Type::FourOfAKind
            }
            (1, Type::FourOfAKind)
            | (2, Type::FullHouse)
            | (3, Type::FullHouse)
            | (4, _)
            | (5, _) => Type::FiveOfAKind,
            (count, t) => panic!("Unknown pairing found: {count} with {t:?}"),
        }
    }
}
