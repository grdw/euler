fn prime_factors(mut number: u64) -> u64 {
    let mut count = 0;
    let mut factor: u64 = 2;
    let mut prev_factor = 0;

    while number > 1 {
        if number % factor == 0 {
            number /= factor;

            if factor != prev_factor {
                count += 1;
            }

            prev_factor = factor;
        } else {
            factor += 1;
        }
    }
    count
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(2), 1);
    assert_eq!(prime_factors(210), 4);
    assert_eq!(prime_factors(644), 3);
    assert_eq!(prime_factors(134043), 4);
    assert_eq!(prime_factors(134044), 4);
}

fn problem_47() -> u64 {
    let mut start: u64 = 1;
    let mut prev_len: u64 = 0;
    let mut count = 0;

    loop {
        start += 1;

        let pf = prime_factors(start);

        if pf != prev_len {
            count = 0
        }

        if pf == 4  {
            count += 1
        }

        if count == 4 {
            break start - count + 1
        }

        prev_len = pf
    }
}

#[test]
fn test_problem_47() {
    assert_eq!(problem_47(), 134043);
}
