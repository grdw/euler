use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

struct Card {
    suit: char,
    value: char
}

impl Card {
    const RADIX: u32 = 10;

    pub fn from_str(s: &str) -> Card {
        Card {
            suit: s.chars().nth(1).unwrap(),
            value: s.chars().nth(0).unwrap()
        }
    }

    pub fn value(&self) -> u32 {
        match self.value {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.value.to_digit(Card::RADIX).unwrap()
            },
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card")
        }
    }
}

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn from_string(hand: String) -> Hand {
        let cards: Vec<_> = hand
            .split_whitespace()
            .map(|card| Card::from_str(card))
            .collect();

        Hand { cards: cards }
    }

    fn total_value(&self) -> u32 {
        self.cards.iter().map(|c| c.value()).sum()
    }

    fn same_suite(&self) -> bool {
        let suit_to_check = self.cards[0].suit;

        self.cards.iter().all(|c| c.suit == suit_to_check)
    }

    fn in_order(&self) -> bool {
        let mut values: Vec<u32> = self.cards
            .iter()
            .map(|c| c.value())
            .collect();

        values.sort();

        values[values.len() - 1] - values[0] == 4
    }

    fn pairs(&self) {
        let mut values: Vec<u32> = self.cards
            .iter()
            .map(|c| c.value())
            .collect();

        values.sort();

        for i in 0..values.len()-1 {
            if values[i] == values[i + 1] {
                println!("{:?}", values[i]);
            }
        }
    }

    fn poker_name(&self) -> (&'static str, i32) {
        self.pairs();
        if self.total_value() == 60 {
            ("ROYAL_FLUSH", 7)
        } else if self.in_order() && self.same_suite() {
            ("STRAIGHT_FLUSH", 6)
        } else {
            ("HIGH_CARD", 0)
        }
    }
}

fn interpret_game(game: String) -> (u32, u32) {
    let p1h = Hand::from_string(String::from(&game[0..5]));
    let p2h = Hand::from_string(String::from(&game[5..10]));

    (0, 0)
}

//#[test]
//fn poker_hands_test() {
//    let f = File::open("src/p054_poker.txt").unwrap();
//    let file = BufReader::new(&f);
//
//    for line in file.lines() {
//        let game = line.unwrap();
//
//        println!("{:?}", interpret_game(game));
//    }
//}
//
//#[test]
//fn parse_game_test() {
//    let game = String::from("8C TS KC 9H 4S 7D 2S 5D 3S AC");
//
//    assert_eq!(interpret_game(game), (0, 1));
//}

#[test]
fn parse_hand_high_card() {
    let hand_high_card = Hand::from_string("8C TS KC 9H 4S".to_string());

    assert_eq!(hand_high_card.total_value(), 44);
}

#[test]
fn parse_hand_one_pair() {
    let hand_one_pair = Hand::from_string("8C 8S KC 9H 4S".to_string());

    assert_eq!(hand_one_pair.total_value(), 42);
}

#[test]
fn parse_hand_two_pairs() {
    let hand_two_pairs = Hand::from_string("8C 8S KC KH 4S".to_string());

    assert_eq!(hand_two_pairs.total_value(), 46);
}

#[test]
fn parse_hand_three_of_a_kind() {
    let hand_three_of_a_kind = Hand::from_string("8C 8S 8H KH 4S".to_string());

    assert_eq!(hand_three_of_a_kind.total_value(), 41);
}

#[test]
fn parse_hand_straight() {
    let straight = Hand::from_string("3C 4S 5H 6H 7S".to_string());

    assert_eq!(straight.total_value(), 25);
}

#[test]
fn parse_hand_flush() {
    let flush = Hand::from_string("4C 5C 1C 6C AC".to_string());

    assert_eq!(flush.total_value(), 30);
}

#[test]
fn parse_hand_full_house() {
    let full_house = Hand::from_string("1A 1C 1S 6C 6H".to_string());

    assert_eq!(full_house.poker_name(), ("FULL_HOUSE", 5));
    assert_eq!(full_house.total_value(), 15);
}

#[test]
fn parse_hand_straight_flush() {
    let straight_flush = Hand::from_string("4C 3C 5C 6C 7C".to_string());

    assert_eq!(straight_flush.poker_name(), ("STRAIGHT_FLUSH", 6));
    assert_eq!(straight_flush.total_value(), 25);
}

#[test]
fn parse_hand_royal_flush() {
    let royal_flush = Hand::from_string("TC JC QC KC AC".to_string());

    assert_eq!(royal_flush.poker_name(), ("ROYAL_FLUSH", 7));
    assert_eq!(royal_flush.total_value(), 60);
}

#[test]
fn card_value_test() {
   let card_t = Card::from_str("TC");
   let card_9 = Card::from_str("9S");
   let card_8h = Card::from_str("8H");

   assert_eq!(card_t.value(), 10);
   assert_eq!(card_9.value(), 9);
   assert_eq!(card_8h.value(), 8);
}
