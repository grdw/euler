// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
//
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
//
// It is possible to make £2 in the following way:
//
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//
// How many different ways can £2 be made using any number of coins?

fn rec_count(coin_counts: &mut Vec<i32>, valid_coins: &Vec<i32>, i: usize, value: i32) -> bool {
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

fn coins_for(value: i32) -> i32 {
    let mut possibilities: i32 = 0;
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

    let mut coin_counts: Vec<i32> = vec![0; valid_coins.len()];
    let mut running = true;

    while running {
        let total: i32 = coin_counts
            .iter()
            .enumerate()
            .map(|(i, x)| valid_coins[i] * x)
            .sum();

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
    assert_eq!(coins_for(200), 0);
}
