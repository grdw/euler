// https://projecteuler.net/problem=758
//
// For large numbers this is really bad. It will be slow as shit because
// that HashSet will grow to infinite and beyond. The fix I guess is to either
// make that HashSet smarter.
// The thing I do see is that there are clear repeating patterns as to how
// to pour the most efficient. But how many repeating patterns are there and
// will they always be there?
//
use std::fs;
use std::cmp;
use std::collections::{VecDeque, HashSet};

// For debugging
const BUCKETNAMES: [char; 3] = ['S', 'M', 'L'];

fn main() {
    println!("Answer: {}", pour_for_primes());
}

struct Fold {
    buckets: [u64; 3],
    depth: u64,
    history: String
}

impl Fold {
    pub fn init(buckets: [u64; 3]) -> Fold {
        Fold {
            buckets,
            depth: 0,
            history: String::new()
        }
    }
}

fn pour_for_primes() -> u64 {
    let p = range_of_primes();
    let mut total = 0;
    for i in 0..2 {
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

#[derive(Clone)]
enum Strategy {
    SLMS,
    LM
}

#[derive(Clone)]
struct Node {
    strat: Strategy,
    goto: usize
}

fn pour_one_litre(s: u64, m: u64) -> u64 {
    let mut answer = 0;
    let l = s + m;
    //let pours: Vec<(usize, usize)> = vec![
    //    (0, 2),
    //    (0, 1),
    //    (1, 2),
    //    (1, 0),
    //    (2, 0),
    //    (2, 1)
    //];

    let n: Vec<Node> = vec![
        Node { strat: Strategy::SLMS, goto: 1 },
        Node { strat: Strategy::LM, goto: 0 },
    ];
    let mut queue = VecDeque::new();
    queue.push_back(Node { strat: Strategy::SLMS, goto: 1 });

    let caps = [s, m, l];
    let mut buckets = [s, m, 0];

    while let Some(node) = queue.pop_front() {
        match node.strat {
            Strategy::SLMS => {
                let t_div = caps[1] / caps[0];
                let div = (caps[2] - buckets[2]) / caps[0];
                let wl = buckets[2] + (caps[0] * div);
                answer += (t_div * 2) + 2;

                buckets[0] = caps[2] - wl;
                buckets[1] = 0;
                buckets[2] = wl;
            },
            Strategy::LM => {
                let pour = cmp::min(buckets[2], caps[1] - buckets[1]);
                buckets[2] -= pour;
                buckets[1] += pour;
                answer += 1;
            }
        }

        if buckets.iter().all(|n| *n != 1) {
            queue.push_back(n[node.goto].clone());
        }
    }
    //let mut h = HashSet::new();
    //let mut current = VecDeque::from([Fold::init(buckets)]);

    //while let Some(fold) = current.pop_front() {
    //    if fold.buckets.iter().any(|v| *v == 1) {
    //        fs::write(format!("{}-{}-{}.csv", s, m, l), fold.history).unwrap();
    //        println!("");
    //        answer = fold.depth;
    //        break
    //    }

    //    if h.contains(&fold.buckets) {
    //        continue
    //    }

    //    for (from, to) in &pours {
    //        if fold.buckets[*from] == 0 { continue }
    //        if fold.buckets[*to] == caps[*to] { continue }

    //        let pour = cmp::min(fold.buckets[*from], caps[*to] - fold.buckets[*to]);
    //        let mut edit = fold.buckets;
    //        edit[*from] -= pour;
    //        edit[*to] += pour;

    //        let mut h = fold.history.clone();
    //        h.push_str(
    //            &format!(
    //                "{},{},{},{},{}\n",
    //                BUCKETNAMES[*from],
    //                BUCKETNAMES[*to],
    //                edit[0],
    //                edit[1],
    //                edit[2]
    //            )
    //        );

    //        current.push_back(
    //            Fold {
    //                buckets: edit,
    //                depth: fold.depth + 1,
    //                history: h
    //            }
    //        );
    //    }

    //    h.insert(fold.buckets);
    //}

    return answer
}

#[test]
fn test_pour_one_litre() {
    assert_eq!(pour_one_litre(63, 33613), 13896);
    assert_eq!(pour_one_litre(485, 33613), 29948);
    //assert_eq!(pour_one_litre(3, 5), 4);
    //assert_eq!(pour_one_litre(7, 31), 20);
    //assert_eq!(pour_one_litre(1234, 4321), 2780);
    // Slow example
    //assert_eq!(pour_one_litre(1964161, 1988017), 619928);
}
