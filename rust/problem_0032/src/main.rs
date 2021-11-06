fn valid_divisors(i: u64, d1: u64, d2: u64) -> bool {
    let mut s = i.to_string();
    s.push_str(&d1.to_string());
    s.push_str(&d2.to_string());

    let mut vector: Vec<char> = s.chars().collect();
    vector.sort();
    vector == vec!['1','2','3','4','5','6','7','8','9']
}

#[test]
fn test_valid_divisors() {
    assert_eq!(valid_divisors(7254, 39, 186), true);
    assert_eq!(valid_divisors(7254, 39, 187), false);
    assert_eq!(valid_divisors(7254, 0, 187), false);
}

fn divisors(i: u64) -> Vec<(u64, u64)> {
    let sqrt = (i as f64).sqrt() as u64;
    let mut total_div = vec![];
    for n in 2..=sqrt {
        if i % n == 0 {
            if !valid_divisors(i, n, i / n) { continue };

            total_div.push((n, i / n));
        }
    }
    total_div
}

#[test]
fn test_common_divisors() {
    assert_eq!(divisors(60), vec![]);
    assert_eq!(divisors(7254), vec![(39, 186)]);
    assert_eq!(divisors(64), vec![]);
    assert_eq!(divisors(6), vec![]);
}

fn problem_32() -> u64 {
    let mut n = 98765;
    let mut all_ns: Vec<u64> = vec![];

    while n > 0 {
        let d = divisors(n);

        if !d.is_empty() {
            all_ns.push(n);
        }
        n -= 1;
    }

    all_ns.sort();
    all_ns.dedup();
    all_ns.iter().sum()
}

#[test]
fn test_problem_32() {
    assert_eq!(problem_32(), 45228);
}
