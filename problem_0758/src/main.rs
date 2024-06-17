// https://projecteuler.net/problem=758
//
use std::cmp;
use std::collections::{VecDeque, HashSet};

// For debugging
const BUCKETNAMES: [char; 3] = ['S', 'M', 'L'];

fn main() {
    println!("Answer: {}", pour_for_primes());
}

fn pour_for_primes() -> u64 {
    let p = range_of_primes();
    let mut total = 0;
    for i in 0..3 {
        println!("=================");
        for j in i+1..4 {
            let s = 2 * p[i].pow(5) - 1;
            let m = 2 * p[j].pow(5) - 1;
            let l = pour_one_litre(s, m);
            println!("{} {} P: {}", s, m, l);

            total += l;
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
    history: Vec<(char, char)>
}

// Thought process:
// I'm starting at depth 2 assume I did 2 moves already: M -> L, S -> M
// The reason for that is that I believe all the examples start out like this
impl Fold {
    pub fn init(buckets: [u64; 3]) -> Fold {
        Fold {
            buckets,
            depth: 0,
            history: vec![]
        }
    }
}

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
    let mut h = HashSet::new();
    let mut current = VecDeque::from([Fold::init(buckets)]);

    while let Some(fold) = current.pop_front() {
        if fold.buckets.iter().any(|v| *v == 1) {
            for (i, (k, v)) in fold.history.iter().enumerate() {
                if i < 100 {
                    print!("{}{}-", k, v);
                }
            }
            println!("");
            answer = fold.depth;
            break
        }

        if h.contains(&fold.buckets) {
            continue
        }

        for (from, to) in &pours {
            if fold.buckets[*from] == 0 { continue }
            if fold.buckets[*to] == caps[*to] { continue }

            let pour = cmp::min(fold.buckets[*from], caps[*to] - fold.buckets[*to]);
            let mut edit = fold.buckets;
            edit[*from] -= pour;
            edit[*to] += pour;

            let mut h = fold.history.clone();
            h.push((BUCKETNAMES[*from], BUCKETNAMES[*to]));
            current.push_back(
                Fold {
                    buckets: edit,
                    depth: fold.depth + 1,
                    history: h
                }
            );
        }

        h.insert(fold.buckets);
    }

    return answer
}

#[test]
fn test_pour_one_litre() {
    assert_eq!(pour_one_litre(3, 5), 4);
    assert_eq!(pour_one_litre(7, 31), 20);
    assert_eq!(pour_one_litre(1234, 4321), 2780);
    // Slow example
    assert_eq!(pour_one_litre(1964161, 1988017), 619928);
}
