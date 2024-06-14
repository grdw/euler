// https://projecteuler.net/problem=758
//

fn main() {
    println!("Answer: {}", pour_for_primes());
}

fn pour_for_primes() -> u64 {
    let p = range_of_primes();
    let mut total = 0;
    for i in 0..p.len() {
        for j in i+1..p.len() {
            let s = 2 * p[i].pow(5) - 1;
            let m = 2 * p[j].pow(5) - 1;
            total += pour_one_litre(s, m);
        }
    }
    return total
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

fn range_of_primes() -> Vec<u64> {
    let mut result = vec![];
    let mut n = 2;
    while n < 1000 {
        if is_prime(n) {
            result.push(n);
        }
        n += 1;
    }
    result
}

fn pour_one_litre(s: u64, m: u64) -> u64 {
    let l = 0;
    return 0
}
