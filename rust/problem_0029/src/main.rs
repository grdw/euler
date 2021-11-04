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

fn prime_factors(mut number: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut factor: u64 = 2;
    let end = (number as f64).sqrt().floor() as u64;

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
    assert_eq!(prime_factors(256), vec![2, 2, 2, 2, 2, 2, 2, 2]);
    assert_eq!(prime_factors(3125), vec![5, 5, 5, 5, 5]);
}

fn problem_29(max: u64) -> u64 {
    let mut size:u64 = 0;
    for i in 2..=max {
        for j in 2..=max {
            let total = vec![i; j as usize];
            println!("{:?}", total);
        }
    }
    0
}

#[test]
fn test_problem_29() {
    for i in 5..7 {
        println!("{:?}", problem_29(i));

    }
    //assert_eq!(problem_29(5), 15);
    //assert_eq!(problem_29(6), 23);
    //assert_eq!(problem_29(10), 69);
}
