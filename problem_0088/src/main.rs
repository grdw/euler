use std::{thread, time::Duration};
use std::collections::HashSet;

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

fn prime_factors(mut number: u64) -> Vec<u64> {
    let mut factor: u64 = 2;
    let mut factors = vec![];

    while number > 1 {
        if is_prime(factor) && number % factor == 0 {
            factors.push(factor);
            number /= factor
        } else {
            factor += 1
        }
    }

    factors
}

fn dfs(nums: &Vec<u64>, i: usize, res: &mut HashSet<Vec<u64>>, subset: &mut Vec<u64>) {
    if i == nums.len() {
        if subset.len() > 1 {
            res.insert(subset.clone());
        }
        return;
    }
    subset.push(nums[i]);
    dfs(nums, i + 1, res, subset);
    subset.pop();
    dfs(nums, i + 1, res, subset);
}

fn factor_groups(p: &Vec<u64>) -> HashSet<Vec<u64>> {
    let mut n = HashSet::new();
    let mut subset = vec![];

    dfs(p, 0, &mut n, &mut subset);
    println!("{:?}", n);

    return n;
}

#[test]
fn test_factor_groups() {
    let l = vec![2,7,3,11,5];
    let answer = HashSet::from([
        // Full set
        vec![2,7,3,11,5],
        // 4 combo's:
        vec![14,3,11,5],
        vec![7,6,11,5],
        vec![7,3,22,5],
        vec![7,3,11,10],
        vec![2,21,11,5],
        vec![2,3,77,5],
        vec![2,3,11,35],
        vec![2,7,33,5],
        vec![2,7,11,15],
        vec![2,7,3,55],
        // 3 combo's:
        vec![42, 11, 5],
        // 2 combo's:
    ]);
    assert_eq!(factor_groups(&l), answer);
    let m = vec![2,2,3,5];
    let answer = HashSet::from([
        vec![2,2,3,5],
        vec![2,2,15],
        vec![2,6,5],
        vec![2,30],
        vec![60],
    ]);
    assert_eq!(factor_groups(&m), answer);
}

fn sum_group(min: u64, max: u64) -> u64 {
    let mut answer = 0;
    let mut s: HashSet<u64> = HashSet::new();

    let mut q = 2;
    let mut k = min;

    'outer: loop {
        //thread::sleep(Duration::from_millis(1000));
        let p = prime_factors(q);

        for g in factor_groups(&p) {
            //println!("{} {:?}", k, g);
            let n = k - g.len() as u64; // this is how many 1's we need to add
            let pr: u64 = g.iter().product();
            let sm: u64 = g.iter().sum::<u64>() + n;

            if pr == sm {
                println!("bingo!");
                k += 1;
                s.insert(pr);

                if k == max + 1 {
                    break 'outer
                }
            }
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
    //assert_eq!(sum_group(2, 2), 4);
    //assert_eq!(sum_group(3, 3), 6);
    //assert_eq!(sum_group(4, 4), 8);
    //assert_eq!(sum_group(5, 5), 8);
    //assert_eq!(sum_group(6, 6), 12);
    assert_eq!(sum_group(10, 10), 16);
    //assert_eq!(sum_group(12000, 12000), 12096);
    //assert_eq!(sum_group(2, 6), 30);
    //assert_eq!(sum_group(2, 12), 61);
    //assert_eq!(sum_group(2, 12000), 7587457);
}
