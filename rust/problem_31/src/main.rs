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
    let mut start = 0;
    let mut cycle = 0;

    loop {
        coin_counts[start] += 1;

        if start > 0 {
            start -= 1;
        }
        //let total: i32 = coin_counts
        //    .iter()
        //    .enumerate()
        //    .map(|(i, c)| valid_coins[i] * c)
        //    .sum();
        println!("{:?}", coin_counts);

        if coin_counts[start] >= value {
            coin_counts[start] = 0;
            start += 1;
        }
    }
    //    if valid_coins[start] <= value {
    //        coin_counts[start] += 1;

    //        for i in 0..start {
    //            coin_counts[i] += 1;
    //        }

    //        //println!("{:?}, {}, {}, {}", coin_counts, total, start, value % total);

    //        if total == value {
    //            coin_counts[start] = 0; // Reset the counter
    //            start += 1;
    //        }

    //        //// A handle bar just in case
    //        //if total > value {
    //        //    break;
    //        //}
    //    }
    //}
    possibilities
}

#[test]
fn build_up_of_coints_test() {
    //assert_eq!(coins_for(1), 1); // There's only 1 coin
    //assert_eq!(coins_for(2), 2); // There's 2 ways
    assert_eq!(coins_for(47), 100); // There's N ways
    //assert_eq!(coins_for(10), 8);
    //assert_eq!(coins_for(13), 8);
    //assert_eq!(coins_for(200), 0);
}
