use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

struct Card {
    suit: char,
    value: char
}

fn hand_value(hand: String) -> u16 {
    let cards: Vec<_> = hand
        .split_whitespace()
        .map(|x| Card { value: x[0], suit: x[1] })
        .collect();

    0
}

fn interpret_game(game: String) -> (u32, u32) {
    let p1h = hand_value(String::from(&game[0..5]));
    let p2h = hand_value(String::from(&game[5..10]));

    println!("{:?}", p1h);

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
fn parse_hand() {
    let hand_high_card = "8C TS KC 9H 4S".to_string();
    let hand_one_pair = "8C 8S KC 9H 4S".to_string();
    let hand_two_pairs = "8C 8S KC KH 4S".to_string();
    let hand_three_of_a_kind = "8C 8S 8H KH 4S".to_string();
    let straight = "3C 4S 5H 6H 7S".to_string();
    let flush = "4C 5C 1C 6C AC".to_string();
    let full_house = "1A 1C 1S 6C 6H".to_string();
    let straight_flush = "3C 4C 5C 6C 7C".to_string();
    let royal_flush = "TC JC QC KC AC".to_string();

    assert_eq!(hand_value(hand_high_card), 0);
    assert_eq!(hand_value(hand_one_pair), 1);
    assert_eq!(hand_value(hand_two_pairs), 2);
    assert_eq!(hand_value(hand_three_of_a_kind), 3);
    assert_eq!(hand_value(straight), 4);
    assert_eq!(hand_value(flush), 5);
    assert_eq!(hand_value(full_house), 6);
    assert_eq!(hand_value(straight_flush), 7);
    assert_eq!(hand_value(royal_flush), 8);
}
