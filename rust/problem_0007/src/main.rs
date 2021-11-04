fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as u64;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn problem_7() -> u64 {
    let mut start = 1;
    let mut index = 0;

    loop {
        start += 1;

        if is_prime(start) {
            index += 1
        }

        if index == 10001 {
            break start
        }
    }
}

#[test]
fn test_problem_7() {
    assert_eq!(problem_7(), 104743);
}
