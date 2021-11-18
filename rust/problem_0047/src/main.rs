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
    let prime_factors = sieve_of_eras();
    let group_size = 4;

    loop {
        let group = &prime_factors[count..count + group_size];

        if group.iter().all(|&d| d == 4) {
            break count as u64
        }

        count += 1;
    }
}

#[test]
fn test_problem_47() {
    assert_eq!(problem_47(), 134043);
}
