fn is_triangular(n: u64) -> bool {
    let t = ((((8 * n) + 1) as f64).sqrt() - 1.0) / 2.0;

    t.fract() == 0.0
}

fn is_pentagonal(i: u64) -> bool {
    let n = ((((24 * i) + 1) as f64).sqrt() + 1.0) / 6.0;

    n.fract() == 0.0
}

fn problem_45() -> u64 {
    let mut n: u64 = 143;
    loop {
        n += 1;

        let m = n * ((2 * n) - 1);

        if is_triangular(m) && is_pentagonal(m) {
            break m
        }
    }
}

#[test]
fn test_problem_45() {
    assert_eq!(problem_45(), 1533776805);
}
