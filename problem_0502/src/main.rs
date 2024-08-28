const MOD: u128 = 1_000_000_007;

fn main() {
    println!("Hello, world!");
}

fn valid_castles(width: u128, height: u128) -> u128 {
    let mut block_count = 1; // This is the bottom block of 'width'
    return 0
}

#[test]
fn test_valid_castles() {
    assert_eq!(valid_castles(4, 2), 10);
    assert_eq!(valid_castles(13, 10), 3729050610636);
    assert_eq!(valid_castles(10, 13), 37959702514);
    assert_eq!(valid_castles(100, 100) % MOD, 841913936);
}
