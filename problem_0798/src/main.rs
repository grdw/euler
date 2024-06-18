fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Card {
    suit: u64,
    card: u64
}

fn game(suits: u64, cards: u64) -> u64 {
    let mut deck = vec![];
    for s in 0..suits {
        for c in 1..=cards {
            deck.push(Card { suit: s, card: c })
        }
    }

    // My thought process here is that if you have 0 visible cards, neither
    // player can put another card on top, automatically making it a 'draw'.
    // The other situation is when the full deck is put on the table, which
    // means that it's an automatic 'draw' as well because there aren't any
    // cards to draw. Not sure why it says "(which can be empty)" because I'm
    // pretty sure it can't with the rules stated.
    let mut picked_cards = 1..(deck.len() - 1);
    let max = deck.len();
    let mut n = 0;
    while n < max {
        let p = n / deck.len();
        println!("--> {} {}", p, n % deck.len());
        //println!("----> {} {}", p, (n / deck.len()) % deck.len());
        n += 1;
    }
    let mut visible_cards: Vec<Card> = vec![];

    println!("{:?}", deck);

    return 0
}

#[test]
fn test_game() {
    assert_eq!(game(2, 3), 26);
}
