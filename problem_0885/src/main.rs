use std::collections::HashMap;

const MOD: u128 = 1123455689;

fn main() {
    println!("Answer: {}", sum(18) % MOD);
}

fn fact(mut i: u32) -> u128 {
    let mut total: u128 = 1;
    while i > 1 {
        total *= i as u128;
        i -= 1;
    }
    total
}

fn sum(length: usize) -> u128 {
    let total = fact(length as u32);
    let max = 9;

    let mut sum: u128 = 0;
    let mut n: Vec<u8> = vec![0; length];
    n[0] = 1;

    loop {
        sum += value_for(&n, total);
        if n.iter().all(|&m| m == max) {
            break;
        }
        n[0] += 1;

        let mut c = 0;
        for m in 0..length-1 {
            if n[m] > max {
                c = m + 1;
                n[c] += 1;
            }
        }

        for t in 0..c {
            n[t] = n[c];
        }
    }

    return sum
}

fn value_for(v: &Vec<u8>, mut t: u128) -> u128 {
    let mut s = HashMap::new();
    let mut q = 0;
    for (i, n) in v.iter().enumerate() {
        s.entry(n)
         .and_modify(|n| *n += 1)
         .or_insert(1);

        q += 10_u128.pow(i as u32) * *n as u128;
    }

    for n in s.values() {
        t /= fact(*n);
    }

    return q * t
}

#[test]
fn test_sum() {
    assert_eq!(sum(1), 45);
    assert_eq!(sum(2), 3465);
    assert_eq!(sum(5), 1543545675);
    assert_eq!(sum(6), 125796691845);
}
