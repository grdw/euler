use std::collections::HashSet;

const MAX: u64 = 50_000_000;

fn main() {
    let mut count = HashSet::new();
    let mut n = 0;
    let l = list_primes(2);
    let k = list_primes(3);
    let m = list_primes(4);
    let max: usize = l.len() * k.len() * m.len();

    while n < max {
        let li = n % l.len();
        let ki = (n / l.len()) % k.len();
        let mi = (n / l.len() / k.len()) % m.len();
        let qq = l[li].pow(2) + k[ki].pow(3) + m[mi].pow(4);

        if qq < MAX {
            count.insert(qq);
        }

        n += 1;
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

fn list_primes(p: u32) -> Vec<u64> {
    let mut n: u64 = 2;
    let mut list = vec![];
    while n.pow(p) < MAX {
        if is_prime(n) {
            list.push(n);
        }
        n += 1;
    }
    list
}
