fn problem_5() -> u64 {
    let mut start = 20;
    loop {
        if (1..=20).all(|x| start % x == 0) {
            break start
        }
        start += 20
    }
}

#[test]
fn test_problem_5() {
    assert_eq!(problem_5(), 232792560);
}
