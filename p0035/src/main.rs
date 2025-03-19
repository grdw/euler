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

fn rotary(number: u64) -> bool {
    let mut chars = number.to_string().chars().collect::<Vec<char>>();

    (0..chars.len()).all(|_| {
        chars.rotate_left(1);

        let rotated: u64 = chars
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        is_prime(rotated)
    })
}

#[test]
fn test_rotary() {
    assert_eq!(rotary(1970), false);
    assert_eq!(rotary(197), true);
    assert_eq!(rotary(19), false);
    assert_eq!(rotary(2), true);
}

fn problem_35() -> usize {
    (1..=1_000_000)
        .filter(|n| is_prime(*n) && rotary(*n))
        .collect::<Vec<u64>>()
        .len()
}

#[test]
fn test_problem_35() {
    assert_eq!(problem_35(), 55)
}
