// https://projecteuler.net/problem=758
//
use std::cmp;
use std::collections::{HashSet, HashMap, VecDeque};

fn main() {
    println!("Answer: {}", pour_for_primes());
}

fn pour_for_primes() -> u64 {
    let p = range_of_primes();
    let mut total = 0;
    for i in 0..p.len() {
        println!("=================");
        for j in i+1..p.len() {
            //let s = 2 * p[i].pow(5) - 1;
            //let m = 2 * p[j].pow(5) - 1;
            println!("{}", pour_one_litre(p[i], p[j]));
            //total += pour_one_litre(s, m);
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
    let mut answer = 0;
    let l = s + m;
    let mut unique_pours = HashSet::new();
    let pours = vec![
        ('S', 'L'),
        ('S', 'M'),
        ('M', 'L'),
        ('M', 'S'),
        ('L', 'S'),
        ('L', 'M')
    ];

    let caps = HashMap::from([
        ('S', s),
        ('M', m),
        ('L', l)
    ]);

    let buckets = HashMap::from([
        ('S', s),
        ('M', m),
        ('L', 0)
    ]);

    let mut current = VecDeque::from([(buckets, 0, 'S', 'L')]);

    while let Some((buckets, depth, prev_f, prev_t)) = current.pop_front() {
        let v = (buckets[&'S'], buckets[&'M'], buckets[&'L']);
        if buckets[&'S'] == 1 || buckets[&'M'] == 1 || buckets[&'L'] == 1 {
            answer = depth;
            break
        }

        if unique_pours.contains(&v) {
            continue
        }

        for (from, to) in &pours {
            if prev_t == *from && prev_f == *to { continue }
            if buckets[from] == 0 { continue }
            if buckets[to] == caps[to] { continue }

            let pour = cmp::min(buckets[from], caps[to] - buckets[to]);
            let mut edit = buckets.clone();

            if let Some(editf) = edit.get_mut(from) {
                *editf -= pour;
            }
            if let Some(editt) = edit.get_mut(to) {
                *editt += pour;
            }

            current.push_back((edit, depth + 1, *from, *to));
        }

        unique_pours.insert(v);
    }

    return answer
}

#[test]
fn test_pour_one_litre() {
    assert_eq!(pour_one_litre(3, 5), 4);
    assert_eq!(pour_one_litre(5, 7), 8);
    assert_eq!(pour_one_litre(7, 9), 12);
    assert_eq!(pour_one_litre(5, 29), 12);
    assert_eq!(pour_one_litre(7, 31), 20);
    assert_eq!(pour_one_litre(1234, 4321), 2780);
    assert_eq!(pour_one_litre((991 * 2) - 1, (997 * 2) - 1), 660);
    //assert_eq!(pour_one_litre((991_u64.pow(2) * 2) - 1, (997_u64.pow(2) * 2) - 1), 619928);
    //assert_eq!(pour_one_litre(1911605485491901, 1970179460809513), 2780);
}
