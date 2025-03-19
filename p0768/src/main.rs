use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn balanced_chandelier(holders: u64, candles: u64) -> u64 {
    if candles > holders { panic!("Impossible") }
    if holders == candles { return 1 }
    if candles == 2 { return holders / candles }

    //println!("{} {}", holders / 2, candles / 2);
    return binomial_coefficient(holders / 2, candles / 2);
}

fn binomial_coefficient(n: u64, k: u64) -> u64 {
    if k < 0 || k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = cmp::min(k, n - k); // Take advantage of symmetry
    let mut c = 1;
    for i in 0..k {
        println!("{} / {} = {}", (n - i), (i + 1), (n - i) / (i + 1));
        c = c * (n - i) / (i + 1);
    }

    c
}

#[test]
fn test_candle_holders_f22() {
    assert_eq!(balanced_chandelier(2, 2), 1);
}

#[test]
fn test_candle_holders_f4_2() {
    assert_eq!(balanced_chandelier(4, 2), 2);
}

#[test]
fn test_candle_holders_f1000_2() {
    assert_eq!(balanced_chandelier(1000, 2), 500);
}

#[test]
fn test_candle_holders_f12_4() {
    assert_eq!(balanced_chandelier(12, 4), 15);
}

#[test]
fn test_candle_holders_f14_4() {
    assert_eq!(balanced_chandelier(14, 4), 21);
}

#[test]
fn test_candle_holders_f16_4() {
    assert_eq!(balanced_chandelier(16, 4), 28);
}

#[test]
fn test_candle_holders_f36_6() {
    assert_eq!(balanced_chandelier(36, 6), 876);
}

#[test]
fn test_candle_holders_f18_4() {
    assert_eq!(balanced_chandelier(18, 4), 36);
}

//#[test]
//fn test_candle_holders_f360_20() {
//    assert_eq!(balanced_chandelier(360, 20), 5);
//}
