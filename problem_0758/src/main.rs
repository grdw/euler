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

struct Fold {
    buckets: [u64; 3],
    depth: u64,
    previous_pours: [(usize, usize); 3],
}

impl Fold {
    pub fn init(buckets: [u64; 3]) -> Fold {
        Fold {
            buckets,
            depth: 0,
            previous_pours: [(0, 0); 3]
        }
    }
}

const BUCKETNAMES: [char; 3] = ['S', 'M', 'L'];

fn pour_one_litre(s: u64, m: u64) -> u64 {
    let mut answer = 0;
    let l = s + m;
    let pours: Vec<(usize, usize)> = vec![
        (0, 2),
        (0, 1),
        (1, 2),
        (1, 0),
        (2, 0),
        (2, 1)
    ];

    let caps = [s, m, l];
    let buckets = [s, m, 0];
    let mut current = VecDeque::from([Fold::init(buckets)]);

    while let Some(fold) = current.pop_front() {
        if fold.buckets.iter().any(|v| *v == 1) {
            answer = fold.depth;
            break
        }

        for (from, to) in &pours {
            if fold.previous_pours
                .iter()
                .any(|(pf, pt)| pf == to && pt == from ) {
                continue
            }
            if fold.buckets[*from] == 0 { continue }
            if fold.buckets[*to] == caps[*to] { continue }

            let pour = cmp::min(fold.buckets[*from], caps[*to] - fold.buckets[*to]);
            let mut edit = fold.buckets;
            edit[*from] -= pour;
            edit[*to] += pour;
            //print!("{} -> {} -> {}", BUCKETNAMES[*from], pour, BUCKETNAMES[*to]);
            //println!(" {:?} {} HISTORY: {:?}", edit, fold.depth, fold.previous_pours);

            let mut previous_pours = fold.previous_pours;
            previous_pours[fold.depth as usize % previous_pours.len()] = (*from, *to);

            current.push_back(
                Fold {
                    buckets: edit,
                    depth: fold.depth + 1,
                    previous_pours,
                }
            );
        }
    }

    return answer
}

#[test]
fn test_pour_one_litre() {
    assert_eq!(pour_one_litre(3, 5), 4);
    //assert_eq!(pour_one_litre(5, 7), 8);
    //assert_eq!(pour_one_litre(7, 9), 12);
    //assert_eq!(pour_one_litre(5, 29), 12);
    //assert_eq!(pour_one_litre(7, 31), 20);
    //assert_eq!(pour_one_litre(1234, 4321), 2780);
    //assert_eq!(pour_one_litre((991 * 2) - 1, (997 * 2) - 1), 660);
    //assert_eq!(pour_one_litre((991_u64.pow(2) * 2) - 1, (997_u64.pow(2) * 2) - 1), 619928);
    //assert_eq!(pour_one_litre(1911605485491901, 1970179460809513), 2780);
}
