use std::{fs, str};
use std::collections::HashMap;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
];

const RANKS: [&'static str; 10] = [
    "royal_flush",
    "straight_flush",
    "four_of_a_kind",
    "full_house",
    "flush",
    "straight",
    "three_of_a_kind",
    "two_pairs",
    "one_pair",
    "high_card"
];


fn card_value(value: char) -> usize {
    CARDS
        .iter()
        .position(|&r| r == value)
        .unwrap()
}

#[derive(Debug)]
struct PokerHand<'a>(Vec<&'a str>);

impl PokerHand<'_> {
    pub fn sorted(mut cards: Vec<&str>) -> PokerHand {
        cards.sort_by_key(|a|
            card_value(a.chars().nth(0).unwrap())
        );

        PokerHand(cards)
    }

    pub fn rank(&self) -> Vec<usize> {
        let mut result = vec![];

        for rank in RANKS {
            let opt = match rank {
                "royal_flush"     => self.is_royal_flush(),
                "straight_flush"  => self.is_straight_flush(),
                "four_of_a_kind"  => self.is_four_of_a_kind(),
                "full_house"      => self.is_full_house(),
                "flush"           => self.is_flush(),
                "straight"        => self.is_straight(),
                "three_of_a_kind" => self.is_three_of_a_kind(),
                "two_pairs"       => self.is_two_pairs(),
                "one_pair"        => self.is_one_pair(),
                "high_card"       => self.is_high_card(),
                _ => panic!("no such method")
            };

            let arr = opt.unwrap_or(vec![]);

            if arr.len() > 0 {
                result = arr;
                break;
            }
        }

        result
    }

    pub fn is_royal_flush(&self) -> Option<Vec<usize>> {
        let check = &CARDS[0..5];
        let cards = &self.0;
        let suite = cards[0].chars().nth(1);

        if cards
            .iter()
            .all(|x|
                 check.contains(&x.chars().nth(0).unwrap())
                    && x.chars().nth(1) == suite
            ) {
                Some(vec![9])
            } else {
                None
            }
    }

    pub fn is_straight_flush(&self) -> Option<Vec<usize>> {
        if self.is_straight().is_some() && self.is_flush().is_some() {
            Some(vec![8])
        } else {
            None
        }
    }

    pub fn is_four_of_a_kind(&self) -> Option<Vec<usize>> {
        self.is_any_of_a_kind(4, 1, 7)
    }

    pub fn is_full_house(&self) -> Option<Vec<usize>> {
        if self.is_three_of_a_kind().is_some() && self.is_one_pair().is_some() {
            Some(vec![6])
        } else {
            None
        }
    }

    pub fn is_flush(&self) -> Option<Vec<usize>> {
        let cards = &self.0;
        let suite = cards[0].chars().nth(1);

        if cards.iter().all(|x| x.chars().nth(1) == suite) {
            Some(vec![5])
        } else {
            None
        }
    }

    pub fn is_straight(&self) -> Option<Vec<usize>> {
        let mut straight = None;
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
                straight = Some(vec![4]);
                break;
            }
            n += 1;
        }

        straight
    }

    pub fn is_three_of_a_kind(&self) -> Option<Vec<usize>> {
        self.is_any_of_a_kind(3, 1, 3)
    }

    pub fn is_two_pairs(&self) -> Option<Vec<usize>> {
        self.is_any_of_a_kind(2, 2, 2)
    }

    pub fn is_one_pair(&self) -> Option<Vec<usize>> {
        self.is_any_of_a_kind(2, 1, 1)
    }

    pub fn is_high_card(&self) -> Option<Vec<usize>> {
        let highest_card = self.0[0].chars().nth(0).unwrap();

        Some(
            vec![
                0, CARDS.len() - card_value(highest_card)
            ]
        )
    }

    fn is_any_of_a_kind(&self, max: u8, n: u8, t: usize) -> Option<Vec<usize>> {
        let mut count = 0;
        let mut any_of_a_kind = None;
        let mut keys = vec![t];
        let mut map: HashMap<char, u8> = HashMap::new();
        let cards = &self.0;

        // Yeah and fuck you to
        for &c in cards {
            let value = c.chars().nth(0).unwrap();
            let counter = map.entry(value).or_insert(0);
            *counter += 1
        }

        let mut map_keys: Vec<char> = map.keys().cloned().collect();
        map_keys.sort();
        map_keys.reverse();

        for key in map_keys {
            let val = map.get(&key).unwrap();

            if *val == max {
                count += 1;
                keys.push(CARDS.len() - card_value(key));

                if count == n {
                    any_of_a_kind = Some(keys);
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
    let rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AD"]);
    let no_rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AS"]);

    assert_eq!(rf_hand.is_royal_flush(), Some(vec![9]));
    assert_eq!(no_rf_hand.is_royal_flush(), None);
    assert_eq!(rf_hand.rank(), vec![9]);
}

#[test]
fn test_poker_hand_wins_from() {
    let rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AD"]).rank();
    let no_rf_hand = PokerHand::sorted(vec!["TD", "JD", "QD", "KD", "AS"]).rank();
    let lose_hand = PokerHand::sorted(vec!["TD", "5Q", "3S", "2D", "AS"]).rank();
    let loser_hand = PokerHand::sorted(vec!["TD", "5Q", "3S", "2D", "KS"]).rank();

    assert!(rf_hand > no_rf_hand);
    assert!(rf_hand > lose_hand);
    assert!(lose_hand > loser_hand);
}

#[test]
fn test_poker_hand_straight_flush() {
    let sf_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KD"]);
    let no_sf_hand_1 = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);
    let no_sf_hand_2 = PokerHand::sorted(vec!["8D", "TD", "JD", "QD", "KD"]);

    assert_eq!(sf_hand.is_straight_flush(), Some(vec![8]));
    assert_eq!(sf_hand.rank(), vec![8]);
    assert_eq!(no_sf_hand_1.is_straight_flush(), None);
    assert_eq!(no_sf_hand_2.is_straight_flush(), None);
}

#[test]
fn test_poker_hand_four_of_a_kind() {
    let foak_hand = PokerHand::sorted(vec!["9D", "5H", "5C", "5S", "5D"]);
    let no_foak_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(foak_hand.is_four_of_a_kind(), Some(vec![7, 4]));
    assert_eq!(foak_hand.rank(), vec![7, 4]);
    assert_eq!(no_foak_hand.is_four_of_a_kind(), None);
}

#[test]
fn test_poker_hand_full_house() {
    let fh_hand = PokerHand::sorted(vec!["9D", "9H", "5C", "5S", "5D"]);
    let no_fh_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(fh_hand.is_full_house(), Some(vec![6]));
    assert_eq!(fh_hand.rank(), vec![6]);
    assert_eq!(no_fh_hand.is_full_house(), None);
}

#[test]
fn test_poker_hand_flush() {
    let f_hand = PokerHand::sorted(vec!["9D", "2D", "3D", "KD", "QD"]);
    let no_f_hand = PokerHand::sorted(vec!["9D", "TD", "JD", "QD", "KS"]);

    assert_eq!(f_hand.is_flush(), Some(vec![5]));
    assert_eq!(f_hand.rank(), vec![5]);
    assert_eq!(no_f_hand.is_flush(), None);
}

#[test]
fn test_poker_hand_straight() {
    let s_hand = PokerHand::sorted(vec!["2D", "3D", "4D", "5S", "6D"]);
    let no_s_hand_1 = PokerHand::sorted(vec!["QD", "8C", "7C", "6C", "5D"]);
    let no_s_hand_2 = PokerHand::sorted(vec!["7D", "TD", "JD", "QD", "KS"]);

    assert_eq!(s_hand.is_straight(), Some(vec![4]));
    assert_eq!(s_hand.rank(), vec![4]);
    assert_eq!(no_s_hand_1.is_straight(), None);
    assert_eq!(no_s_hand_2.is_straight(), None);
}

#[test]
fn test_poker_hand_two_pairs() {
    let tp_hand = PokerHand::sorted(vec!["2S", "2D", "3D", "3S", "5D"]);
    let no_tp_hand = PokerHand::sorted(vec!["7D", "TD", "JD", "QD", "KS"]);

    assert_eq!(tp_hand.is_two_pairs(), Some(vec![2, 2, 1]));
    assert_eq!(tp_hand.rank(), vec![2, 2, 1]);
    assert_eq!(no_tp_hand.is_two_pairs(), None);
}

#[test]
fn test_poker_hand_high_card() {
    let h_hand = PokerHand::sorted(vec!["2S", "3D", "4C", "5S", "7D"]);

    assert_eq!(h_hand.rank(), vec![0, 6]);
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
        let p1 = hand_to_cards(&game[0..14]);
        let p2 = hand_to_cards(&game[15..]);

        if p1.rank() > p2.rank() {
            wins += 1
        }
    }

    wins
}

#[test]
fn test_problem_54() {
    assert_eq!(problem_54(), 376);
}
