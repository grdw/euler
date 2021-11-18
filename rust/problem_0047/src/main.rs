fn sieve_of_eras() -> Vec<u64> {
    let n = 1_000_000;
    let mut prime_factors = vec![0; n + 1];

    for i in 2..=n {
        if prime_factors[i] == 0 {
            let mut j = i;
            while j <= n {
                prime_factors[j] += 1;
                j += i
            }
        }
    }

    prime_factors
}

fn problem_47() -> u64 {
    let mut count = 0;
    let mut prev_count = 0;
    let mut solution = 0;
    let prime_factors = sieve_of_eras();

    for (i, prime_count) in prime_factors.iter().enumerate() {
        if *prime_count != prev_count {
            count = 0;
        }

        if *prime_count == 4 {
            count += 1;
        }

        if count == 4 {
            solution = i - 3;
            break;
        }

        prev_count = *prime_count;
    }

    solution as u64
}

#[test]
fn test_problem_47() {
    assert_eq!(problem_47(), 134043);
}
