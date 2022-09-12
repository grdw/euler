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

fn problem_60(size: usize) -> u64 {
    let mut max = 100_000;
    let mut group = vec![2];
    let mut index = 0;
    let mut reset_count = 0;

    loop {
        group[index] = next_prime(group[index]);

        if is_prime_pair_set(&group) {
            if index == size - 1 {
                let total = group.iter().sum();

                println!("{:?} -> {}", group, total);
                if total < max {
                    max = total;
                    for i in 0..index { group.pop(); }
                    index = 0;
                }

                continue;
            }

            let next_prime = next_prime(group[index]);
            group.push(next_prime);
            index += 1;
        }

        if group.iter().sum::<u64>() > max {
            println!("RESET! {:?}", group);
            for i in 1..index { group.pop(); }
            index = 1;
            reset_count += 1;

            if reset_count > 1 {
                print!("FULL \n");
                group.pop();
                index = 0;
                reset_count = 0;
            }
        } else {
            reset_count = 0;
        }

        if group[0] > max {
            break;
        }
    }

    max
}

#[test]
fn test_problem_60() {
    assert_eq!(problem_60(5), 792);
    //assert_eq!(problem_60(5), 792);
}
