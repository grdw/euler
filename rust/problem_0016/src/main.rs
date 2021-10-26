fn main() {
    println!("Hello, world!");
}

use std::cmp;

const MAX_POWER: u32 = 100;

fn int_to_vec(mut number: u128) -> Vec<u8> {
    let mut result = vec![];
    let mut tens: u128 = 10;

    while i > 0 {
        let base = number % tens;
        result.push((base / (tens / 10)) as u8);

        tens *= 10;
        number -= base;
    }
    result
}

fn multiply(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];

    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let mut total = vec![0; i + j];
            let mut mul_vec = int_to_vec((x * y) as u128);

            total.append(&mut mul_vec);
            result = sum_arrays(result, total);
        }
    }

    result
}

fn sum_arrays(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let len: usize = cmp::max(a.len(), b.len());
    let mut result = vec![0; len];

    for i in 0..len {
        let a_val = a.get(i).unwrap_or(&0);
        let b_val = b.get(i).unwrap_or(&0);
        let total = int_to_vec((a_val + b_val) as u128);

        for (j, t) in total.iter().enumerate() {
            let k = i + j;

            match result.get_mut(k) {
                Some(n) => *n += t,
                None => result.insert(k, *t)
            }
        }
    }

    result
}

fn problem_16(power: u32) -> u16 {
    let mut result = vec![1];
    let mut cycles = power / MAX_POWER;
    let rest = power % MAX_POWER;

    while cycles > 0 {
        result = multiply(result, int_to_vec(2_u128.pow(MAX_POWER)));
        cycles -= 1
    }

    if rest > 0 {
        result = multiply(result, int_to_vec(2_u128.pow(rest)));
    }

    result.iter().fold(0, |t, x| t + *x as u16)
}

#[test]
fn test_problem_16() {
    assert_eq!(problem_16(15), 26);
    assert_eq!(problem_16(115), 164);
    assert_eq!(problem_16(1000), 1366);
}

#[test]
fn test_int_to_vec() {
    assert_eq!(int_to_vec(16), vec![6, 1]);
    assert_eq!(int_to_vec(128), vec![8, 2, 1])
}

#[test]
fn test_summing_arrays() {
    assert_eq!(
        sum_arrays(vec![], vec![0, 8]),
        vec![0, 8]
    );
    assert_eq!(
        sum_arrays(vec![0, 2, 1], vec![0, 8]),
        vec![0, 0, 2]
    );
    assert_eq!(
        sum_arrays(vec![2, 9], vec![0, 8]),
        vec![2, 7, 1]
    );
}

#[test]
fn test_summing_multiple_arrays() {
    let mut start = vec![1, 1, 1];
    start = sum_arrays(start, vec![1, 1, 2]);
    start = sum_arrays(start, vec![1, 1, 2]);

    assert_eq!(start, vec![3, 3, 5]);
}

#[test]
fn test_multiply() {
    assert_eq!(
        multiply(
            int_to_vec(16),
            int_to_vec(28)
        ),
        vec![8, 4, 4]
    )
}
