fn rec_count(coin_counts: &mut Vec<u32>, valid_coins: &Vec<u32>, i: usize, value: u32) -> bool {
    let mut running = i <= (coin_counts.len() - 1);

    if running {
        coin_counts[i] += 1;

        if coin_counts[i] >= (value / valid_coins[i]) + 1 {
            coin_counts[i] = 0;
            running = rec_count(coin_counts, valid_coins, i + 1, value);
        }
    }
    running
}

fn coins_for(value: u32) -> u32 {
    let mut possibilities: u32 = 0;
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

    let mut coin_counts: Vec<u32> = vec![0; valid_coins.len()];
    let mut running = true;
    let mut total: u32;

    while running {
        total = coin_counts
            .iter()
            .enumerate()
            .fold(0u32, |sum, (i, count)| sum + (valid_coins[i] * count));

        if total == value {
            possibilities += 1;
        }

        running = rec_count(&mut coin_counts, &valid_coins, 0, value);
    }

    possibilities
}

#[test]
fn build_up_of_coins_test() {
    assert_eq!(coins_for(1), 1); // There's only 1 coin
    assert_eq!(coins_for(2), 2); // There's 2 ways
    assert_eq!(coins_for(4), 3); // There's N ways
    assert_eq!(coins_for(10), 11);
    assert_eq!(coins_for(13), 16);
    assert_eq!(coins_for(99), 4366);
    //assert_eq!(coins_for(200), 0); // TOO SLOW!
}
