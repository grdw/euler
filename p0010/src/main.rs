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

fn total_primes_for(num: i64) -> i64 {
    let mut total = 0;
    for i in 0..num {
        if is_prime(i) {
            total += i;
        }
    }
    total
}

#[test]
fn total_primes_below_x() {
    assert_eq!(total_primes_for(10), 17);
    assert_eq!(total_primes_for(2_000_000), 142913828922);
}
