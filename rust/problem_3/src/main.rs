fn is_prime(number: i64) -> bool {
    let end = (number as f64).sqrt().floor() as i64;
    let vec: Vec<i64> = (2..end+1).collect();

    number > 1 && vec.iter().all(|i|
        *i == number || number % i > 0
    )
}

fn prime_factor(number: i64) -> i64 {
    let mut factors = vec![];
    let l = number as f64;

    for i in 2..number {
        let k = i as f64;
        let m = (l / k).floor() as i64;

        if is_prime(m) && number % m == 0 && ! factors.contains(&m) {
            factors.push(m);
        }
    }

    *factors.iter().max().unwrap_or(&-1)
}

#[test]
fn is_prime_tests() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(1151), true);
    assert_eq!(is_prime(6228), false);
}

#[test]
fn prime_factors_test() {
    assert_eq!(prime_factor(13195), 29);
    //assert_eq!(prime_factor(600851475143), 1);
}
