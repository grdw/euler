use std::collections::HashSet;

const MAX: u64 = 50_000_000;

fn main() {
    let primes = list_primes();
    let mut count = HashSet::new();

    for x in &primes {
        for y in &primes {
            for z in &primes {
                let n = x.pow(2) + y.pow(3) + z.pow(4);
                if n < MAX {
                    count.insert(n);
                }
            }
        }
    }
    println!("Answer: {:?}", count.len());
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

fn list_primes() -> Vec<u64> {
    let mut n: u64 = 2;
    let mut list = vec![];
    while n.pow(2) < MAX {
        if is_prime(n) {
            list.push(n);
        }
        n += 1;
    }
    list
}
