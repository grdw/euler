fn main() {
    println!("Hello, world!");
}

fn problem_63() -> u128 {
    let mut count = 0;
    let mut start: u128 = 1;
    let mut pow = 1;
    let mut prev_count = 0;

    loop {
        let n = start.pow(pow);
        let n_len = ((n as f64).log10() + 1.0) as u32;

        if pow == n_len {
            count += 1;
        } else if n_len > pow {
            start = 1;
            pow += 1;

            if prev_count == count {
                break;
            }

            prev_count = count;
        }

        start += 1;
    }

    count
}

#[test]
fn test_problem_63() {
    assert_eq!(problem_63(), 49);
}
