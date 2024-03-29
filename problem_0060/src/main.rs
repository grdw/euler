use std::collections::HashSet;

fn main() {
    let answer = problem_60(5);
    println!("The lowest sum is: {}", answer);
}

fn is_prime(number: u64, cache: &mut HashSet<u64>) -> bool {
    if number < 2 {
        return false
    }

    if cache.get(&number).is_some() {
        return true;
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as u64;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }

    if is_prime {
        cache.insert(number);
    }

    is_prime
}

fn is_prime_pair_set(primes: &Vec<u64>, cache: &mut HashSet<u64>) -> bool {
    for i in 0..primes.len() {
        for j in (i+1)..primes.len() {
            let first_concat = format!("{}{}", primes[i], primes[j])
                .parse::<u64>()
                .unwrap();

            if !is_prime(first_concat, cache) {
                return false
            }

            let second_concat = format!("{}{}", primes[j], primes[i])
                .parse::<u64>()
                .unwrap();

            if !is_prime(second_concat, cache) {
                return false
            }
        };
    };

    true
}

#[test]
fn test_prime_pair_set() {
    let mut cache = HashSet::new();

    assert_eq!(
        is_prime_pair_set(&vec![3, 11, 109, 673], &mut cache),
        false
    );

    assert_eq!(
        is_prime_pair_set(&vec![3, 7, 109, 673], &mut cache),
        true
    );

    assert_eq!(
        is_prime_pair_set(&vec![2, 3, 5, 7, 11], &mut cache),
        false
    );
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

    primes[5] = false; // Ugly hack, but 5 can't count because of the division rule.
    primes
}

fn find_next(mut value: u64, sieve: &Vec<bool>) -> u64 {
    if value < 3 {
        return 3
    }

    let mut found = false;

    while !found {
        value += 1;

        if sieve[value as usize] {
            found = true;
        }
    }

    return value
}


fn problem_60(size: usize) -> u64 {
    let primes = sieve_of_erato(1_000_000);
    let mut cache = HashSet::new();
    let mut upper_bound = 100_000;
    let mut group = vec![0];
    let mut index = 0;

    loop {
        match group.get(index) {
            Some(_) => group[index] = find_next(group[index], &primes),
            None => group.push(find_next(group[index - 1], &primes))
        }

        let total = group.iter().sum();

        if is_prime_pair_set(&group, &mut cache) {
            index += 1;

            if group.len() == size && total < upper_bound {
                index = 0;
                upper_bound = total;
            }
        }

        if group[0] > upper_bound {
            break;
        }

        // Piss poor reset function
        if total >= upper_bound {
            if index < 3 {
                group.truncate(1);
                index = 0;
            } else {
                group.truncate(2);
                index = 1;
            }
        }
    }

    upper_bound
}

#[test]
fn test_problem_60_example() {
    assert_eq!(problem_60(4), 792);
}

#[test]
fn test_problem_60_real_deal() {
    assert_eq!(problem_60(5), 26033);
}
