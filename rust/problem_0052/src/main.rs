fn to_digits(number: u64) -> Vec<u8> {
    let mut digits = vec![];
    let mut digit_length = (number as f64).log10().ceil() as u32;

    while digit_length > 0 {
        let tens = 10_u64.pow(digit_length);
        let base = number % tens;
        let digit = (base / (tens / 10)) as u8;

        digits.push(digit);
        digit_length -= 1
    }

    digits.sort();
    digits
}

fn problem_52() -> u64 {
    let mut x = 1;

    loop {
        let dx = to_digits(x);

        if (2..=6).all(|m| to_digits(x * m) == dx) {
            break;
        }

        x += 1
    }

    x
}

#[test]
fn test_problem_52() {
    assert_eq!(problem_52(), 142857);
}
