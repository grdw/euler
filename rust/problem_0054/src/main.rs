use std::{fs, str};
use std::collections::HashMap;
use std::cmp::Ordering;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
];

fn card_value(value: char) -> usize {
    CARDS
        .iter()
        .position(|&r| r == value)
        .unwrap()
}

#[derive(Debug)]
struct PokerHand<'a>(Vec<&'a str>, Vec<char>);

struct PokerHandIter<'a> {
    inner: &'a PokerHand<'a>,
    index: usize
}

impl PartialOrd for PokerHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        0.partial_cmp(&2)
    }
}

impl PartialEq for PokerHand<'_> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Iterator for PokerHandIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.index >= self.inner.1.len() {
            None
        } else {
            let card = self.inner.1[self.index];
            let value = card_value(card);

            self.index += 1;
            Some(CARDS.len() - value)
        }
    }
}

impl PokerHand<'_> {
    pub fn sorted(mut cards: Vec<&str>) -> PokerHand {
        cards.sort_by_key(|a|
            card_value(a.chars().nth(0).unwrap())
        );

        let mut values = vec![];
        values.push(cards[0].chars().nth(0).unwrap());

        PokerHand(cards, values)
    }

    pub fn iter<'a>(&'a self) -> PokerHandIter<'a> {
        PokerHandIter {
            inner: self,
            index: 0
        }
    }

    pub fn rank(&mut self) -> u16 {
        self.1.push(self.0[0].chars().nth(0).unwrap());

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
        let check = &CARDS[0..5];
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

    pub fn is_four_of_a_kind(&mut self) -> bool {
        self.is_any_of_a_kind(4, 1)
    }

    pub fn is_full_house(&mut self) -> bool {
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
        let mut straight = false;
        let cards = &self.0;

        let values: Vec<char> = cards
            .iter()
            .map(|c| c.chars().nth(0).unwrap())
            .collect();

        let mut n = 0;
        let max = CARDS.len() - 5;

        while max >= n {
            let con_slice = &CARDS[n..n + 5];
            if values == con_slice {
                straight = true;
                break;
            }
            n += 1;
        }

        straight
    }

    pub fn is_three_of_a_kind(&mut self) -> bool {
        self.is_any_of_a_kind(3, 1)
    }

    pub fn is_two_pairs(&mut self) -> bool {
        self.is_any_of_a_kind(2, 2)
    }

    pub fn is_one_pair(&mut self) -> bool {
        self.is_any_of_a_kind(2, 1)
    }

    fn is_any_of_a_kind(&mut self, max: u8, n: u8) -> bool {
        let mut count = 0;
        let mut any_of_a_kind = false;
        let mut map: HashMap<char, u8> = HashMap::new();
        let cards = &self.0;

        for &c in cards {
            let value = c.chars().nth(0).unwrap();
            let counter = map.entry(value).or_insert(0);
            *counter += 1
        }

        for (key, val) in map {
            if val == max {
                count += 1;

                if count == n {
                    self.1.insert(0, key);

                    any_of_a_kind = true;
                    break
                }
            }
        }

        any_of_a_kind
    }
}

#[test]
fn test_poker_hand_sorted() {
    let hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AD"]);

    assert_eq!(hand.0, vec!["AD", "KD", "QD", "JD", "TD"])
}

#[test]
fn test_poker_hand_royal_flush() {
    let mut rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AD"]);
    let no_rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AS"]);

    assert_eq!(rf_hand.is_royal_flush(), true);
    assert_eq!(no_rf_hand.is_royal_flush(), false);
    assert_eq!(rf_hand.rank(), 9);
}

#[test]
fn test_poker_hand_royal_flush_high() {
    let rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AD"]);
    let no_rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AS"]);

    assert_eq!(rf_hand > no_rf_hand, true);
}

#[test]
fn test_poker_hand_straight_flush() {
    let mut sf_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KD"]);
    let no_sf_hand_1 = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);
    let no_sf_hand_2 = PokerHand::sorted(vec!["8D", "TD", "JD", "QD", "KD"]);

    assert_eq!(sf_hand.is_straight_flush(), true);
    assert_eq!(sf_hand.rank(), 8);
    assert_eq!(no_sf_hand_1.is_straight_flush(), false);
    assert_eq!(no_sf_hand_2.is_straight_flush(), false);
}

#[test]
fn test_poker_hand_four_of_a_kind() {
    let mut foak_hand = PokerHand::sorted(vec!["9D", "5H", "5C", "5S", "5D"]);
    let mut no_foak_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(foak_hand.is_four_of_a_kind(), true);
    assert_eq!(foak_hand.rank(), 7);
    assert_eq!(no_foak_hand.is_four_of_a_kind(), false);
}

#[test]
fn test_poker_hand_full_house() {
    let mut fh_hand = PokerHand::sorted(vec!["9D", "9H", "5C", "5S", "5D"]);
    let mut no_fh_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(fh_hand.is_full_house(), true);
    assert_eq!(fh_hand.rank(), 6);
    assert_eq!(no_fh_hand.is_full_house(), false);
}

#[test]
fn test_poker_hand_flush() {
    let mut f_hand = PokerHand::sorted(vec!["9D", "2D", "3D", "KD", "QD"]);
    let no_f_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(f_hand.is_flush(), true);
    assert_eq!(f_hand.rank(), 5);
    assert_eq!(no_f_hand.is_flush(), false);
}

#[test]
fn test_poker_hand_straight() {
    let mut s_hand = PokerHand::sorted(vec!["2D", "3D", "4D", "5S", "6D"]);
    let no_s_hand_1 = PokerHand::sorted(vec!["QD", "8C", "7C", "6C", "5D"]);
    let no_s_hand_2 = PokerHand::sorted(vec!["7D", "TD", "JD", "QD", "KS"]);

    assert_eq!(s_hand.is_straight(), true);
    assert_eq!(s_hand.rank(), 4);
    assert_eq!(no_s_hand_1.is_straight(), false);
    assert_eq!(no_s_hand_2.is_straight(), false);
}

#[test]
fn test_poker_hand_two_pairs() {
    let mut tp_hand = PokerHand::sorted(vec!["2S", "2D", "3D", "3S", "5D"]);
    let mut no_tp_hand = PokerHand::sorted(vec!["7D", "TD", "JD", "QD", "KS"]);

    assert_eq!(tp_hand.is_two_pairs(), true);
    assert_eq!(tp_hand.rank(), 2);
    assert_eq!(no_tp_hand.is_two_pairs(), false);
}

#[test]
fn test_poker_hand_high_card() {
    let mut h_hand = PokerHand::sorted(vec!["2S", "3D", "4C", "5S", "7D"]);

    assert_eq!(h_hand.rank(), 0);
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

    PokerHand::sorted(cards)
}

fn problem_54() -> u16 {
    let contents = fs::read_to_string("p054_poker.txt")
                      .unwrap_or("".to_string());

    let mut wins = 0;
    let mut games: Vec<&str> = contents.split("\n").collect();
    // For some reason the last entry in games is a blank line
    games.remove(games.len() - 1);

    for game in &games {
        let mut v1 = hand_to_cards(&game[0..14]);
        let mut v2 = hand_to_cards(&game[15..]);

        let v1_rank = v1.rank();
        let v2_rank = v2.rank();

        if v1_rank > v2_rank {
            wins += 1
        } else if v1_rank == v2_rank {
            let mut v1_iter = v1.iter();
            let mut v2_iter = v2.iter();

            loop {
                match (v1_iter.next(), v2_iter.next()) {
                    (Some(p1), Some(p2)) => {
                        if p1 > p2 {
                            wins += 1;
                            break;
                        }

                        if p2 > p1 {
                            break;
                        }
                    },
                    (_, _) => break
                }
            }
        }
    }

    wins
}

#[test]
fn test_problem_54() {
    assert_eq!(problem_54(), 376);
}
