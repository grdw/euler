pub trait ToVector {
    fn to_vec(&self) -> Vec<u8>;
}

impl ToVector for u128 {
    fn to_vec(&self) -> Vec<u8> {
        let mut number = *self;
        let mut result = vec![];
        let mut tens: u128 = 10;

        while number > 0 {
            let base = number % tens;
            result.push((base / (tens / 10)) as u8);

            tens *= 10;
            number -= base;
        }
        result
    }
}

fn fact(mut i: u8) -> u128 {
    let mut total: u128 = 1;
    while i > 1 {
        total *= i as u128;
        i -= 1;
    }
    total
}

#[test]
fn test_fact() {
    assert_eq!(fact(6), 720);
    assert_eq!(fact(16), 20922789888000);
}

fn problem_34() -> u128 {
    let mut total = 0;
    let mut start = 10;
    let facts: Vec<u128> = (0..=9).map(|n| fact(n)).collect();

    loop {
        let digits = start.to_vec();
        let sum: u128 = digits
            .iter()
            .map(|n| facts[*n as usize])
            .sum();

        if sum == start {
            total += start
        }

        // Knowing 9!*7 is the highest amount of digits to choose where
        // the sum of its factorials will remain the 'length' of 7.
        // Why not pick an upper bound where the first number has
        // a length of 7?
        if digits.len() == 7 && digits.len() == sum.to_vec().len() {
            break
        }

        start += 1;
    }
    total
}

#[test]
fn test_problem_34() {
    assert_eq!(problem_34(), 40730);
}
