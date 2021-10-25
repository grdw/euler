//fn main() {
//    println!("Hello, world!");
//}

fn chain_length(mut n: u64) -> u64 {
    let mut count = 1;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n = 3 * n + 1
        }
        count += 1
    }
    count
}

fn problem_14() -> u64 {
    (2..=1_000_000)
        .map(|n| (n, chain_length(n)))
        .max_by_key(|n| n.1)
        .unwrap().0
}

#[test]
fn test_max_chain_length() {
    assert_eq!(problem_14(), 837799)
}

#[test]
fn test_chain_length() {
    assert_eq!(chain_length(13), 10)
}
