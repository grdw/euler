use std::collections::HashSet;

fn main() {
    println!("Answer: {}", sum_group(2, 12000));
}

// The way this works is that you take a number f.e. 8 and a group size of 4
// The function starts at the bottom. By taking the number 8, a divisor 2,
// the size of the group - 1 (as we're taking one factor off). The other
// two arguments are the sum (0) and product (1).
//
// The loop starts at 2 till 8. It checks which of these number are
// divisible by 8. Which will be 2 and 4.
//
// The product will be determined which 1 * 2 = 2 and 1 * 4 = 4.
// The sum will be 0 + 2 = 2 and 0 + 4 = 4.
// The group size in this example, as specified is, 4. This means that the
// factors will look like [2] and [4] but the rest will be all 1's.
// For the product there will be no changes since multiplying by 1 till Infinity
// will always return the same number. For summing there will be a difference,
// which is why we're counting the remainder of factors and adding that to
// the total sum. We'll then compare as the original assignment states
// if the product and sum are equal, and if they are, return true!
// if not we go further, and recursively check the rest of the factors which
// in the case of 8 will look like:
//
// 8 -- 4 -- 2
//  \
//   2
//    \
//     2
//      \
//       2
//
// 1 is not counted which is why that i > 1 is there.
// For [4,2,1,1] or [4,2] (+2 for summation). The product and sum match to 8.
//
fn get_multiplication_partitions(n: u64, k: u64) -> bool {
    fn partitions(n: u64, start: u64, k: u64, s: u64, p: u64) -> bool {
        for i in start..=n {
            if n % i == 0 && i > 1 {
                let pp = p * i;
                let ss = s + i;

                if ss + k == pp {
                    return true
                }

                if k > 0 && partitions(n / i, i, k - 1, ss, pp) {
                    return true
                }
            }
        }
        return false
    }

    return partitions(n, 2, k - 1, 0, 1);
}

#[test]
fn test_multiplication_partitions() {
    //assert_eq!(get_multiplication_partitions(3, 2), false);
    //assert_eq!(get_multiplication_partitions(4, 2), true);
    //assert_eq!(get_multiplication_partitions(12, 6), true);
    assert_eq!(get_multiplication_partitions(8, 5), true);
    //assert_eq!(get_multiplication_partitions(12096, 12000), true);
}

fn sum_group(min: u64, max: u64) -> u64 {
    let mut answer = 0;
    let mut q = min;
    let mut k = min;
    let mut s: HashSet<u64> = HashSet::new();

    while k <= max {
        if get_multiplication_partitions(q, k) {
            k += 1;
            s.insert(q);
            q = k;
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
}
