fn partial_coeff(mut n: u64, mut m: u64) -> bool {
    let mut total: f64 = 1.0;

    loop {
        total *= n as f64 / m as f64;

        if total > 1_000_000.0 {
            break true
        }

        n -= 1;
        m -= 1;

        if m < 1 {
            break false
        }
    }
}

#[test]
fn test_partial_coeff() {
    assert_eq!(partial_coeff(100, 3), false);
    assert_eq!(partial_coeff(23, 10), true)
}

fn problem_53() -> u64 {
    let mut count = 0;

    for r in 1..=100 {
        for n in r + 1..=100 {
            let div = if (n - r) > r {
                r
            } else {
                n - r
            };

            if partial_coeff(n, div) {
                count += 1
            }
        }
    }
    count
}

#[test]
fn test_problem_53() {
    assert_eq!(problem_53(), 4075);
}
