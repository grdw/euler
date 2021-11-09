fn is_prime(number: i64) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as i64;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn is_truncatable_prime(number: i64) -> bool {
    if !is_prime(number) || number < 10 {
        return false;
    }

    let mut truncatable = true;
    let n = number.to_string();

    for i in 0..n.len() {
        let r_slice: i64 = n[0..i+1].parse().unwrap_or(-1);
        let l_slice: i64 = n[i..n.len()].parse().unwrap_or(-1);

        if !is_prime(r_slice) || !is_prime(l_slice) {
            truncatable = false;
            break;
        }
    }

    truncatable
}

const MAX_TRUNCATABLES: i32 = 11;

fn truncatable_primes() -> i64 {
    let mut sum = 0;
    let mut n = 0;
    let mut i = 0;

    loop {
        i += 1;

        if is_truncatable_prime(i) {
            n += 1;
            sum += i;
        }

        if n >= MAX_TRUNCATABLES {
            break;
        }
    }

    sum
}

#[test]
fn is_truncatable_test() {
    assert_eq!(is_truncatable_prime(4), false);
    assert_eq!(is_truncatable_prime(2), false);
    assert_eq!(is_truncatable_prime(3), false);
    assert_eq!(is_truncatable_prime(5), false);
    assert_eq!(is_truncatable_prime(7), false);
    assert_eq!(is_truncatable_prime(11), false);
    assert_eq!(is_truncatable_prime(233), false);
    assert_eq!(is_truncatable_prime(4733), false);
    assert_eq!(is_truncatable_prime(3797), true);
}

#[test]
fn truncatable_primes_test() {
    assert_eq!(truncatable_primes(), 748317);
}
