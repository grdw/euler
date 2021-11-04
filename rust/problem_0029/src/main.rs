fn main() {
    println!("Hello, world!");
}

fn is_prime(number: u8) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as u8;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn prime_factors(mut number: u8) -> Vec<u8> {
    let mut factors: Vec<u8> = vec![];
    let mut factor: u8 = 2;
    let end = (number as f64).sqrt().floor() as u8;

    while number > 1 {
        if is_prime(factor) && number % factor == 0 {
            factors.push(factor);
            number /= factor;
        } else {
            factor += 1;
        }
    }
    factors
}

#[test]
fn test_prime_factors() {
    assert_eq!(prime_factors(2), vec![2]);
    assert_eq!(prime_factors(3), vec![3]);
    assert_eq!(prime_factors(4), vec![2, 2]);
    assert_eq!(prime_factors(5), vec![5]);
    assert_eq!(prime_factors(10), vec![2, 5]);
    assert_eq!(prime_factors(99), vec![3, 3, 11]);
    assert_eq!(prime_factors(100), vec![2, 2, 5, 5]);
}

fn problem_29(max: u16) -> u16 {
    let mut totals: Vec<String> = vec![];
    for a in 2..=max {
        for b in 2..=max {
            let primes = prime_factors(a as u8);
            let mut string = String::from("");

            for i in 0..primes.len() {
                let sub_string = primes[i].to_string().repeat(b as usize);
                string.push_str(&sub_string);
            }

            if !totals.contains(&string) {
                totals.push(string);
            }
        }
    }
    println!("{:?}", totals);
    totals.len() as u16
}

#[test]
fn test_problem_29() {
    assert_eq!(problem_29(5), 15);
    assert_eq!(problem_29(6), 23);
    assert_eq!(problem_29(100), 9183);
}
