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

fn problem_30() -> u128 {
    let mut matches = vec![];
    for n in 2..=999999 {
        let result: u128 = n.to_vec()
            .iter()
            .map(|d| (*d as u128).pow(5))
            .fold(0, |dp, acc| acc + dp);

        if n == result {
            matches.push(n);
        }
    }
    matches.iter().fold(0, |p, acc| *acc + p)
}

#[test]
fn test_power_digits() {
    assert_eq!(problem_30(), 443839);
}
