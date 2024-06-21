use std::collections::VecDeque;
fn main() {
    println!("Answer: {}", detract_all(10_u128.pow(17)));
}

// The thought process is like this:
// F.e. 100 takes 512 total steps to form all values from 1 till 100
// To make 1 till 7 a total of 28 steps are required as it's
// 7 + 6 + 5 + 4 + 3 + 2 + 1
// To make 8 till 26 a total of these steps are required:
// 1..8, 2..9, 3..5 (it ends at 5 steps because of 27)
// fact_sum(8) + fact_sum(9) - fact_sum(1) + fact_sum(5) - fact_sum(2)
//
fn detract_all(n: u128) -> u128 {
    let comb = all_cubes(n);
    println!("{:?}", comb);
    let mut steps = 0;
    for i in 0..n {
        println!("{}, {}", i, lame(i, &comb));
        steps += lame(i, &comb);
    }
    return steps
}

fn lame(mut n: u128, cubes: &Vec<u128>) -> u128 {
    let mut steps = 0;
    while n > 0 {
        let mut t = 0;
        for c in cubes.iter().rev() {
            if n / c > 0 {
                t = *c;
                break;
            }
        }

        n -= t;
        steps += 1;
    }

    return steps
}

#[test]
fn test_lame() {
    assert_eq!(lame(100, &vec![1, 8, 27, 64]), 4);
}

#[test]
fn test_detract_all() {
    assert_eq!(detract_all(100), 512);
    assert_eq!(detract_all(1000), 6432);
}

fn all_cubes(n: u128) -> Vec<u128> {
    let mut results = vec![];
    let mut m: u128 = 1;
    loop {
        let p = m.pow(3);
        if p > n {
            break;
        }

        results.push(p);
        m += 1;
    }
    return results;
}
