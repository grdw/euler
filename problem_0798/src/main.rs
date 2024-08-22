fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Card {
    suit: u64,
    value: u64
}

fn game(suits: u64, max_value: u64, visible_cards: u64) -> u64 {
    let mut deck = make_deck(suits, max_value);
    let mut visible: Vec<Card> = vec![];


    println!("{:?}", deck);

    return 0
}

fn make_deck(suits: u64, max_value: u64) -> Vec<Card> {
    let mut deck = vec![];
    for s in 0..suits {
        for v in 1..=max_value {
            deck.push(Card { suit: s, value: v })
        }
    }
    return deck
}

#[test]
fn test_game() {
    assert_eq!(game(2, 3, 2), 26);
}
