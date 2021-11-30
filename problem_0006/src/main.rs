fn problem_6() -> u128 {
    let mut sum: u128 = 0;
    let mut n_sum: u128 = 0;
    for n in 1..100 {
        sum += (n as u128).pow(2);
        n_sum += n;
    }
    n_sum.pow(2) - sum
}

#[test]
fn test_problem_6() {
    assert_eq!(problem_6(), 24174150);
}
