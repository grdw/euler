fn coins_for(value: i32) -> i32 {
    let valid_coins = vec![
        1,   // 1p
        2,   // 2p
        5,   // 5p
        10,  // 10p
        20,  // 20p
        50,  // 50p
        100, // 1P
        200  // 2P
    ];

    let mut c: Vec<i32> = vec![0; value as usize];
    c.insert(0, 1);

    let len: i32 = c.len() as i32;

    for coin in valid_coins {
        for i in 0..(len-coin) {
            c[(i + coin) as usize] += c[i as usize]
        }
    }

    c[c.len() - 1]
}

#[test]
fn build_up_of_coins_test() {
    assert_eq!(coins_for(1), 1); // There's only 1 coin
    assert_eq!(coins_for(2), 2); // There's 2 ways
    assert_eq!(coins_for(4), 3); // There's N ways
    assert_eq!(coins_for(10), 11);
    assert_eq!(coins_for(13), 16);
    assert_eq!(coins_for(99), 4366);
    assert_eq!(coins_for(200), 73682);
}
