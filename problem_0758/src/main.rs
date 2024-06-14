// https://projecteuler.net/problem=758
//
use std::cmp;
use std::collections::{HashSet, VecDeque};

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

use std::{thread, time::Duration};

fn pour_one_litre(s: u64, m: u64) -> u64 {
    let mut answer = 0;
    let l = s + m;
    let mut current = VecDeque::from([(s, m, 0, 0)]);
    let mut unique_pours = HashSet::new();
    let pours = vec![
        "SL",
        "SM",
        "ML",
        "MS",
        "LS",
        "LM"
    ];

    while let Some((sb, mb, lb, depth)) = current.pop_front() {
        if sb == 1 || mb == 1 || lb == 1 {
            answer = depth;
            break
        }

        if unique_pours.contains(&(sb, mb, lb)) {
            continue
        }

        for p in &pours {
            match *p {
                "SL" => {
                    if sb == 0 { continue }
                    if lb == l { continue }
                    let pour = cmp::min(sb, l - lb);

                    current.push_back((sb - pour, mb, lb + pour, depth + 1));
                },
                "SM" => {
                    if sb == 0 { continue }
                    if mb == m { continue }
                    let pour = cmp::min(sb, m - mb);

                    current.push_back((sb - pour, mb + pour, lb, depth + 1));
                },
                "ML" => {
                    if mb == 0 { continue }
                    if lb == l { continue }
                    let pour = cmp::min(mb, l - lb);

                    current.push_back((sb, mb - pour, lb + pour, depth + 1));
                },
                "MS" => {
                    if mb == 0 { continue }
                    if sb == s { continue }
                    let pour = cmp::min(mb, s - sb);

                    current.push_back((sb + pour, mb - pour, lb, depth + 1));
                },
                "LS" => {
                    if lb == 0 { continue }
                    if sb == s { continue }
                    let pour = cmp::min(lb, s - sb);

                    current.push_back((sb + pour, mb, lb - pour, depth + 1));
                },
                "LM" => {
                    if lb == 0 { continue }
                    if mb == m { continue }
                    let pour = cmp::min(lb, m - mb);

                    current.push_back((sb, mb + pour, lb - pour, depth + 1));
                },
                _ => continue
            }
        }

        unique_pours.insert((sb, mb, lb));
    }

    return answer
}

#[test]
fn test_pour_one_litre() {
    assert_eq!(pour_one_litre(3, 5), 4);
    assert_eq!(pour_one_litre(7, 31), 20);
    assert_eq!(pour_one_litre(1234, 4321), 2780);
}
