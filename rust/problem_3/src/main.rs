fn is_prime(number: i64) -> bool {
    let end = (number as f64).sqrt().floor() as i64;
    let vec: Vec<i64> = (2..end+1).collect();

    number > 1 && vec.iter().all(|i|
        *i == number || number % i > 0
    )
}

fn prime_factor(number: i64) -> i64 {
    let mut highest_fac: i64 = 0;
    let mut factor: i64 = 2;

    loop {
        factor += 1;

        if !is_prime(factor) {
            continue;
        }

        if number % factor == 0 && highest_fac < factor {
            highest_fac = factor;
        }

        if factor >= number {
            break
        }
    }
    highest_fac
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
