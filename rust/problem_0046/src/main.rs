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

fn is_prime_and_square(n: u64, primes: &Vec<u64>) -> bool {
    let mut goldbachs_conjecture = false;
    let mut squares = vec![];

    for p in 1..=((n / 2) as f64).sqrt() as u64 {
        squares.push(p.pow(2) * 2);
    }

    'outer: for p in primes {
        let t = n - p;

        for ds in &squares {
            if t < *ds { continue };

            if t - ds == 0 {
                goldbachs_conjecture = true;
                break 'outer;
            }
        }
    }

    goldbachs_conjecture
}

#[test]
fn test_is_prime_and_square() {
    let primes = vec![2, 3, 5, 7, 11, 13];

    assert_eq!(is_prime_and_square(9, &primes), true);
    assert_eq!(is_prime_and_square(15, &primes), true);
    assert_eq!(is_prime_and_square(25, &primes), true);
}

fn problem_46() -> u64 {
    let mut start = 2;
    let mut primes = vec![];

    loop {
        if start % 2 == 1 &&
           !is_prime(start) &&
           !is_prime_and_square(start, &primes) {
            break start
        }

        if is_prime(start) {
            primes.push(start);
        }

        start += 1;
    }
}

#[test]
fn test_problem_46() {
    assert_eq!(problem_46(), 5777)
}
