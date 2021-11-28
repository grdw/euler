use std::{fs, str};

#[derive(Debug)]
struct PokerHand<'a>(Vec<&'a str>);

impl PokerHand<'_> {
    pub fn value(&self) -> u16 {
        if self.is_royal_flush() {
            9
        } else if self.is_straight_flush() {
            8
        } else {
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
        let mut straight_flush = true;
        let check = vec![
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
        ];
        let cards = &self.0;
        let suite = cards[0].chars().nth(1);
        let len = cards.len();
        let mut prev_position = None;

        for i in 0..len {
            let mut card = cards[i];

            if card.chars().nth(1) != suite {
                straight_flush = false;
                break
            }

            let value = card.chars().nth(0).unwrap();
            let position = check.iter().position(|&v| v == value);

            if prev_position.is_some() {
                let prev_val = prev_position.unwrap();
                let pos_val = position.unwrap();

                if pos_val - prev_val > 1 {
                    straight_flush = false;
                    break;
                }
            }
            prev_position = position
        }
        straight_flush
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
