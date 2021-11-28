use std::{fs, str};
use std::collections::HashMap;

#[derive(Debug)]
struct PokerHand<'a>(Vec<&'a str>);

impl PokerHand<'_> {
    pub fn value(&self) -> u16 {
        if self.is_royal_flush() {
            9
        } else if self.is_straight_flush() {
            8
        } else if self.is_four_of_a_kind() {
            7
        } else if self.is_full_house() {
            6
        } else if self.is_flush() {
            5
        } else if self.is_straight() {
            4
        } else if self.is_three_of_a_kind() {
            3
        } else if self.is_two_pairs() {
            2
        } else if self.is_one_pair() {
            1
        } else { // This is 'high card'
            0
        }
    }

    pub fn is_royal_flush(&self) -> bool {
        let check = vec!['T', 'J', 'Q', 'K', 'A'];
        let cards = &self.0;
        let suite = cards[0].chars().nth(1);

        cards
            .iter()
            .all(|x|
                 check.contains(&x.chars().nth(0).unwrap())
                    && x.chars().nth(1) == suite
            )
    }

    pub fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    pub fn is_four_of_a_kind(&self) -> bool {
        self.is_any_of_a_kind(4, 1)
    }

    pub fn is_full_house(&self) -> bool {
        self.is_three_of_a_kind() && self.is_one_pair()
    }

    pub fn is_flush(&self) -> bool {
        let cards = &self.0;
        let suite = cards[0].chars().nth(1);

        cards
            .iter()
            .all(|x| x.chars().nth(1) == suite)
    }

    pub fn is_straight(&self) -> bool {
        let mut straight = true;
        let check = vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
        ];
        let cards = &self.0;
        let len = cards.len();
        let mut prev_position = None;

        for i in 0..len {
            let card = cards[i];
            let value = card.chars().nth(0).unwrap();
            let position = check.iter().position(|&v| v == value);

            if prev_position.is_some() {
                let prev_val = prev_position.unwrap();
                let pos_val = position.unwrap();

                if pos_val > prev_val && pos_val - prev_val > 1 {
                    straight = false;
                    break;
                }
            }
            prev_position = position
        }
        straight
    }

    pub fn is_three_of_a_kind(&self) -> bool {
        self.is_any_of_a_kind(3, 1)
    }

    pub fn is_two_pairs(&self) -> bool {
        self.is_any_of_a_kind(2, 2)
    }

    pub fn is_one_pair(&self) -> bool {
        self.is_any_of_a_kind(2, 1)
    }

    fn is_any_of_a_kind(&self, max: u8, n: u8) -> bool {
        let mut any_of_a_kind = false;
        let mut map: HashMap<char, u8> = HashMap::new();
        let cards = &self.0;
        let mut count = 0;

        for &c in cards {
            let value = c.chars().nth(0).unwrap();
            let counter = map.entry(value).or_insert(0);
            *counter += 1
        }

        for val in map.values() {
            if *val == max {
                count += 1;

                if count == n {
                    any_of_a_kind = true;
                    break
                }
            }
        }

        any_of_a_kind
    }
}

#[test]
fn test_poker_hand_royal_flush() {
    let rf_hand = PokerHand(vec!["TD", "JD", "QD", "KD", "AD"]);
    let no_rf_hand = PokerHand(vec!["TD", "JD", "QD", "KD", "AS"]);

    assert_eq!(rf_hand.is_royal_flush(), true);
    assert_eq!(no_rf_hand.is_royal_flush(), false);
    assert_eq!(rf_hand.value(), 9);
}

#[test]
fn test_poker_hand_straight_flush() {
    let sf_hand = PokerHand(vec!["9D", "TD", "JD", "QD", "KD"]);
    let no_sf_hand_1 = PokerHand(vec!["9D", "TD", "JD", "QD", "KS"]);
    let no_sf_hand_2 = PokerHand(vec!["8D", "TD", "JD", "QD", "KD"]);

    assert_eq!(sf_hand.is_straight_flush(), true);
    assert_eq!(sf_hand.value(), 8);
    assert_eq!(no_sf_hand_1.is_straight_flush(), false);
    assert_eq!(no_sf_hand_2.is_straight_flush(), false);
}

#[test]
fn test_poker_hand_four_of_a_kind() {
    let foak_hand = PokerHand(vec!["9D", "5H", "5C", "5S", "5D"]);
    let no_foak_hand = PokerHand(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(foak_hand.is_four_of_a_kind(), true);
    assert_eq!(foak_hand.value(), 7);
    assert_eq!(no_foak_hand.is_four_of_a_kind(), false);
}

#[test]
fn test_poker_hand_full_house() {
    let fh_hand = PokerHand(vec!["9D", "9H", "5C", "5S", "5D"]);
    let no_fh_hand = PokerHand(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(fh_hand.is_full_house(), true);
    assert_eq!(fh_hand.value(), 6);
    assert_eq!(no_fh_hand.is_full_house(), false);
}

#[test]
fn test_poker_hand_flush() {
    let f_hand = PokerHand(vec!["9D", "2D", "3D", "KD", "QD"]);
    let no_f_hand = PokerHand(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(f_hand.is_flush(), true);
    assert_eq!(f_hand.value(), 5);
    assert_eq!(no_f_hand.is_flush(), false);
}

#[test]
fn test_poker_hand_straight() {
    let s_hand = PokerHand(vec!["1D", "2D", "3D", "4S", "5D"]);
    let no_s_hand = PokerHand(vec!["7D", "TD", "JD", "QD", "KS"]);

    assert_eq!(s_hand.is_straight(), true);
    assert_eq!(s_hand.value(), 4);
    assert_eq!(no_s_hand.is_straight(), false);
}

#[test]
fn test_poker_hand_two_pairs() {
    let tp_hand = PokerHand(vec!["2S", "2D", "3D", "3S", "5D"]);
    let no_tp_hand = PokerHand(vec!["7D", "TD", "JD", "QD", "KS"]);

    assert_eq!(tp_hand.is_two_pairs(), true);
    assert_eq!(tp_hand.value(), 2);
    assert_eq!(no_tp_hand.is_two_pairs(), false);
}

fn hand_to_cards(hand: &str) -> PokerHand {
    let mut cards = vec![];
    let mut p = 0;
    let max = hand.len();

    while p >= 0 {
        let card = &hand[p..=p + 1];
        cards.push(card);
        p += 3;

        if p > max {
            break;
        }
    }

    PokerHand(cards)
}

fn value_hand(hand: &str) -> u16 {
    let cards = hand_to_cards(hand);
    println!("{:?}", cards);
    0
}

#[test]
fn test_value_hand() {
    assert_eq!(value_hand("5H 5C 6S 7S KD"), 1)
}

fn problem_54() -> u16 {
    let contents = fs::read_to_string("p054_poker.txt")
                      .unwrap_or("".to_string());

    let mut wins = 0;
    let mut games: Vec<&str> = contents.split("\n").collect();
    // For some reason the last entry in games is a blank line
    games.remove(games.len() - 1);

    for game in &games {
        let p1 = &game[0..14];
        let p2 = &game[15..];

        if value_hand(p1) > value_hand(p2) {
            wins += 1
        }
    }

    wins
}

#[test]
fn test_problem_54() {
    assert_eq!(problem_54(), 1);
}
