use std::{thread, time::Duration};
use std::collections::{HashSet, HashMap};

fn main() {
    println!("Answer: {}", sum_group(2, 12000));
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

fn prime_factors(mut number: u64) -> HashMap<u64, u64> {
    let mut factor: u64 = 2;
    let mut factors = HashMap::new();

    while number > 1 {
        if is_prime(factor) && number % factor == 0 {
            factors
                .entry(factor)
                .and_modify(|n| *n += 1)
                .or_insert(1);

            number /= factor
        } else {
            factor += 1
        }
    }

    factors
}

fn factor_groups(p: &HashMap<u64, u64>, h: &mut u64, k: u64) {
    let mut t = 0;
    let mut pr = 1;
    let mut sm = 0;

    for (key, value) in p {
        pr *= key.pow(*value as u32);
        sm += key * value;
        t += value;
    }

    sm += k - t;

    if pr == sm {
        *h = pr
    } else if t > 2 {
        let mut v: Vec<u64> = vec![];
        for (key, value) in p {
            let m = vec![key; *value as usize];
            v.extend(m);
        }
        for n in 0..v.len() {
            for m in (n + 1)..v.len() {
                let mut pc = p.clone();
                let left = v[n];
                let right = v[m];
                pc.entry(left).and_modify(|n| *n -= 1);
                pc.entry(right).and_modify(|n| *n -= 1);
                pc.entry(left * right).and_modify(|n| *n += 1).or_insert(1);
                println!("{:?}", pc);
                factor_groups(&pc, h, k);
            }
        }
    }
}

#[test]
fn test_factor_groups() {
    let mut s = 0;
    factor_groups(&prime_factors(4), &mut s, 2);
    assert_eq!(s, 4);

    let mut s = 0;
    factor_groups(&prime_factors(12), &mut s, 6);
    assert_eq!(s, 12);

    let mut s = 0;
    factor_groups(&prime_factors(11), &mut s, 6);
    assert_eq!(s, 0);
}

fn sum_group(min: u64, max: u64) -> u64 {
    let mut answer = 0;
    let mut s: HashSet<u64> = HashSet::new();

    let mut q = min;
    let mut k = min;

    'outer: loop {
        let p = prime_factors(q);
        let mut hh = 0;
        println!("{}", q);
        factor_groups(&p, &mut hh, k);

        if hh > 0 {
            println!("bingo!");
            k += 1;
            s.insert(hh);
        }

        if k == max + 1 {
            break 'outer
        }

        q += 1;
    }

    for v in &s {
        answer += v;
    }

    return answer
}

#[test]
fn test_sum_group() {
    assert_eq!(sum_group(2, 2), 4);
    assert_eq!(sum_group(3, 3), 6);
    assert_eq!(sum_group(4, 4), 8);
    assert_eq!(sum_group(5, 5), 8);
    assert_eq!(sum_group(6, 6), 12);
    assert_eq!(sum_group(10, 10), 16);
    //assert_eq!(sum_group(12000, 12000), 12096);
    //assert_eq!(sum_group(2, 6), 30);
    //assert_eq!(sum_group(2, 12), 61);
    //assert_eq!(sum_group(2, 12000), 7587457);
}
