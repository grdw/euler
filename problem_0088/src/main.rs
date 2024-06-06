use std::collections::HashSet;

fn main() {
    println!("Answer: {}", sum_group(2, 12000));
}

fn get_multiplication_partitions(n: u64, k: u64) -> bool {
    let mut found = false;

    fn partitions(m: u64, n: u64, start: u64, k: u64, s: u64, p: u64, t: &mut bool) {
        for i in start..=n {
            if n % i == 0 && i > 1 {
                let pp = p * i;
                let ss = s + i + k;

                if ss == pp && m == pp {
                    *t = true;
                }

                if k > 0 {
                    partitions(m, n / i, i, k - 1, s + i, p * i, t);
                }
            }
        }
    }

    partitions(n, n, 2, k - 1, 0, 1, &mut found);
    return found
}

#[test]
fn test_multiplication_partitions() {
    assert_eq!(get_multiplication_partitions(3, 2), false);
    assert_eq!(get_multiplication_partitions(4, 2), true);
    assert_eq!(get_multiplication_partitions(12, 6), true);
    assert_eq!(get_multiplication_partitions(8, 5), true);
}

fn sum_group(min: u64, max: u64) -> u64 {
    let mut answer = 0;
    let mut q = min;
    let mut k = min;
    let mut s: HashSet<u64> = HashSet::new();

    loop {
        if get_multiplication_partitions(q, k) {
            k += 1;
            if k % 100 == 0 {
                println!("at k = {}", k);
            }
            s.insert(q);
            q = min;
        }

        if k == max + 1 {
            break;
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
    assert_eq!(sum_group(2, 2), 4);
    assert_eq!(sum_group(3, 3), 6);
    assert_eq!(sum_group(4, 4), 8);
    assert_eq!(sum_group(4, 5), 8);
    assert_eq!(sum_group(6, 6), 12);
    assert_eq!(sum_group(10, 10), 16);
    assert_eq!(sum_group(12, 12), 16);
    assert_eq!(sum_group(12000, 12000), 12096);
    assert_eq!(sum_group(2, 6), 30);
    assert_eq!(sum_group(2, 12), 61);
    //assert_eq!(sum_group(2, 12000), 7587457);
}
