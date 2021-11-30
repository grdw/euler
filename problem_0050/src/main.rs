fn sieve_of_erato(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n + 1];
    let max = (n as f64).sqrt() as usize;

    for i in 2..=max {
        if primes[i] {
            let mut j = i.pow(2);
            while j <= n {
                primes[j] = false;
                j += i
            }
        }
    }

    primes
}

#[test]
fn test_sieve_of_erato() {
    let sieves = sieve_of_erato(20);
    assert_eq!(sieves[2], true);
    assert_eq!(sieves[4], false);
}

const MAX_N: usize = 1_000_000;

fn problem_50() -> u64 {
    // The puzzle specifies that 953 is made up of 21 consecutive primes so
    // we'll start from 21.
    let mut chain_length = 21;
    let mut max_prime = 0;

    let sieve = sieve_of_erato(MAX_N);
    let mut primes = vec![];

    for n in 2..sieve.len() {
        if sieve[n] {
            primes.push(n);
        }
    }

    loop {
        let max_l = primes.len() - chain_length;
        let mut sum: usize = primes[0..chain_length].iter().sum();

        // If the initial sum goes over 1m, stop!
        if sum > MAX_N {
            break;
        }

        for p in 1..max_l {
            sum += primes[p - 1 + chain_length];
            sum -= primes[p - 1];

            if sum < sieve.len() && sieve[sum] {
                max_prime = sum;
            }
        }

        chain_length += 1;
    }

    max_prime as u64
}

#[test]
fn test_problem_50() {
    assert_eq!(problem_50(), 997651);
}
