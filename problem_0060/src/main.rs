fn main() {
    println!("Hello, world!");
}

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

fn next_prime(mut n: u64) -> u64 {
    if n <= 1 {
        return 2
    }

    let mut found = false;

    while !found {
        n += 1;

        if is_prime(n) {
            found = true;
        }
    }

    return n
}

#[test]
fn test_next_prime() {
    assert_eq!(next_prime(0), 2);
    assert_eq!(next_prime(5), 7);
    assert_eq!(next_prime(7), 11);
}

fn is_prime_pair_set(primes: &Vec<u64>) -> bool {
    for i in 0..primes.len() {
        for j in (i+1)..primes.len() {
            let first_concat = format!("{}{}", primes[i], primes[j]).parse::<u64>().unwrap();
            let second_concat = format!("{}{}", primes[j], primes[i]).parse::<u64>().unwrap();

            if !is_prime(first_concat) {
                return false
            }

            if !is_prime(second_concat) {
                return false
            }
        };
    };

    true
}

#[test]
fn test_prime_pair_set() {
    assert_eq!(is_prime_pair_set(&vec![3, 11, 109, 673]), false);
    assert_eq!(is_prime_pair_set(&vec![3, 7, 109, 673]), true);
    assert_eq!(is_prime_pair_set(&vec![2, 3, 5, 7, 11]), false);
}

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

fn find_next(mut value: u64, sieve: &Vec<bool>) -> u64 {
    for i in (value as usize)+1..sieve.len() {
        value += 1;

        if sieve[i] {
            return i as u64
        }
    }

    return 0
}

fn problem_60(size: usize) -> u64 {
    let primes = sieve_of_erato(1_000_000);
    let mut upper_bound = 100_000;
    let mut group = vec![2];
    let mut index = 0;

    loop {
        group[index] = find_next(group[index], &primes);

        if is_prime_pair_set(&group) {
            if group.len() == size {
                break;
            }

            group.push(find_next(group[index], &primes));
            index += 1;
        }

        println!("{:?}", group);
        // Piss poor reset function
        if group.iter().sum::<u64>() > upper_bound {
            for _ in 2..group.len() {
                group.pop();
            }

            index = 1;
        }
    }

    group.iter().sum()
}

#[test]
fn test_problem_60_example() {
    assert_eq!(problem_60(4), 792);
}

#[test]
fn test_problem_60_real_deal() {
    assert_eq!(problem_60(5), 792);
}
