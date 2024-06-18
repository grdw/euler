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

#[derive(Clone, Debug)]
enum Step {
    SLMS,
    LM,
    ML,
    SMLS,
}

#[derive(Clone)]
struct Strategy {
    steps: Vec<Step>
}

fn pour_one_litre(s: u64, m: u64) -> u64 {
    let mut answer = u64::MAX;
    let l = s + m;

    let strats: Vec<Strategy> = vec![
        Strategy { steps: vec![Step::SLMS, Step::LM] },
        Strategy { steps: vec![Step::ML, Step::SMLS] }
    ];

    let caps = [s, m, l];

    for strat in &strats {
        let mut buckets = [s, m, 0];
        let mut strat_answer = 0;
        let mut step_index = 0;

        while buckets.iter().all(|n| *n != 1) {
            let step = &strat.steps[step_index];
            println!("B: {:?} {} {:?}", buckets, strat_answer, step);
            match step {
               // In this case you keep on filling L till M is empty
               Step::SLMS => {
                   let t_div = caps[1] / caps[0];
                   let div = (caps[2] - buckets[2]) / caps[0];
                   let wl = buckets[2] + (caps[0] * div);
                   strat_answer += (t_div * 2) + 2;

                   buckets[0] = caps[2] - wl;
                   buckets[1] = 0;
                   buckets[2] = wl;
                   step_index = 1;
               },
               // In this case you keep on filling M till L is empty
               Step::SMLS => {
                   let t_div = caps[1] / caps[0];
                   let div = buckets[2] / caps[0];
                   let wl = buckets[2] - (caps[0] * div);
                   strat_answer += (t_div * 2) + 2;

                   buckets[0] = caps[2] - caps[1] - wl;
                   buckets[1] = caps[1];
                   buckets[2] = wl;
                   step_index = 0;
               },
                Step::LM => {
                    pour(&mut buckets, caps, 2, 1);
                    strat_answer += 1;
                    step_index = 0;
                },
                Step::ML => {
                    pour(&mut buckets, caps, 1, 2);
                    strat_answer += 1;
                    step_index = 1;
                }
            }

            println!("A: {:?} {}", buckets, strat_answer);
        }

        if strat_answer < answer {
            answer = strat_answer
        }
    }

    return answer
}

fn pour(buckets: &mut [u64; 3], caps: [u64; 3], i: usize, j: usize) {
    let pour = cmp::min(buckets[i], caps[j] - buckets[j]);
    buckets[i] -= pour;
    buckets[j] += pour;
}

#[test]
fn test_pour_one_litre_a() {
    assert_eq!(pour_one_litre(3, 5), 4);
}

#[test]
fn test_pour_one_litre_b() {
    assert_eq!(pour_one_litre(7, 31), 20);
}

#[test]
fn test_pour_one_litre_c() {
    assert_eq!(pour_one_litre(1234, 4321), 2780);
}

#[test]
fn test_pour_one_litre_d() {
    assert_eq!(pour_one_litre(63, 33613), 13896);
}

#[test]
fn test_pour_one_litre_e() {
    assert_eq!(pour_one_litre(485, 33613), 29948);
    // Slow example
    //assert_eq!(pour_one_litre(1964161, 1988017), 619928);
}
