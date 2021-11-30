fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as u64;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn max_primes(a: i64, b: i64) -> u64 {
    let mut n: i64 = 0;
    let mut count = 0;

    loop {
        let sum = n.pow(2) + (a * n) + b;

        if is_prime(sum.abs() as u64) {
            count += 1;
        } else {
            break count;
        }
        n += 1;
    }
}

#[test]
fn test_max_primes() {
    assert_eq!(max_primes(1, 41), 40);
    assert_eq!(max_primes(-79, 1601), 80);
    assert_eq!(max_primes(79, 1601), 1);
    assert_eq!(max_primes(1, -59), 1);
}

fn problem_27() -> i64 {
    let mut max = 0;
    let mut max_product = 0;
    let pos_neg = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

    for a in 1..1000 {
        for b in 1..=1000 {
            for (p1, p2) in &pos_neg {
                let current_max = max_primes(a * p1, b * p2);

                if current_max > max {
                    max = current_max;
                    max_product = (a * p1) * (b * p2);
                }
            }
        }
    }

    max_product
}

#[test]
fn test_problem_27() {
    assert_eq!(problem_27(), -59231);
}
