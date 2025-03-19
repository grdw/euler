fn champernowne(max: u64) -> Vec<u64> {
    let mut s = String::from("");

    for n in 1..=max {
        s.push_str(&n.to_string());

        if s.len() >= max as usize {
            break;
        }
    }

    s.chars()
        .map(|n| n.to_digit(10).unwrap() as u64)
        .collect()
}

#[test]
fn test_champernowne() {
    assert_eq!(champernowne(12), vec![1,2,3,4,5,6,7,8,9,1,0,1,1]);
    assert_eq!(champernowne(12).len(), 13);
}

fn problem_40() -> u64 {
    let c = champernowne(1_000_000);

    c[0] * c[9] * c[99] * c[999] * c[9999] * c[99_999] * c[999_999]
}

#[test]
fn test_problem_40() {
    assert_eq!(problem_40(), 210);
}
