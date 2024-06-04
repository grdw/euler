use std::collections::HashSet;

fn main() {
    println!("Answer: {}", sum_group(2, 12000));
}

fn sum_group(min: u64, max: u64) -> u64 {
    let mut answer = 0;
    let mut s: HashSet<u64> = HashSet::new();

    for n in min..=max {
        let p = sum_l(n);
        println!("{} {}", n, p);
        s.insert(p);
    }

    for v in &s {
        answer += v;
    }

    return answer
}

fn sum_l(length: u64) -> u64 {
    let answer: u64;
    let mut stack = vec![];
    let mut number = length;

    'outer: loop {
        stack.push((0, 1, length, number, 2));

        while let Some((sum, prod, l, r, start)) = stack.pop() {
            let mut i = start;
            while i * i <= r {
                if r % i == 0 {
                    stack.push((sum + i, prod * i, l - 1, r / i, i));
                }
                i += 1;
            }

            if sum == 0 || prod == 0 {
                continue
            }

            if (sum + l) == prod {
                answer = prod;
                break 'outer;
            }
        }

        number += 1;
    }

    answer
}

#[test]
fn test_sum_l() {
    assert_eq!(sum_l(2), 4);
    assert_eq!(sum_l(3), 6);
    assert_eq!(sum_l(4), 8);
    assert_eq!(sum_l(5), 8);
    assert_eq!(sum_l(6), 12);
    assert_eq!(sum_l(10), 16);
    assert_eq!(sum_l(12000), 12096);
}

#[test]
fn test_sum_group() {
    assert_eq!(sum_group(2, 2), 4);
    assert_eq!(sum_group(4, 4), 8);
    assert_eq!(sum_group(2, 6), 30);
    assert_eq!(sum_group(2, 12), 61);
    assert_eq!(sum_group(2, 12000), 7587457);
}
