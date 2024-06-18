fn main() {
    println!("Answer: {}", sum(18));
}

fn sum(length: u32) -> u128 {
    let max = 1;
    let mut n = 10_u64.pow(length);
    let mut sum = 0;

    while n > max {
        sum += num_to_sorted_digits(n);
        n -= 1;
    }
    return sum
}

fn num_to_sorted_digits(n: u64) -> u128 {
    let mut p: Vec<char> = n
        .to_string()
        .chars()
        .filter(|&n| n != '0')
        .collect();

    p.sort();
    p.into_iter().collect::<String>().parse::<u128>().unwrap()
}

#[test]
fn test_num_to_sorted_digits() {
    assert_eq!(num_to_sorted_digits(3034), 334);
}
#[test]
fn test_sum() {
    assert_eq!(sum(1), 45);
    assert_eq!(sum(5), 1543545675);
    assert_eq!(sum(6), 125796691845);
}
