fn to_digits(number: u32) -> Vec<u8> {
    let mut digits = vec![];
    let mut digit_length = ((number as f64).log10() + 1.0).floor() as u32;

    while digit_length > 0 {
        let tens = 10_u32.pow(digit_length);
        let base = number % tens;
        let digit = (base / (tens / 10)) as u8;

        digits.push(digit);
        digit_length -= 1
    }

    digits
}

pub trait VecEx<U8> {
    fn sum_vec(&mut self, total: &Vec<u8>);
}

impl VecEx<u8> for Vec<u8> {
    fn sum_vec(&mut self, total: &Vec<u8>) {
        let mut prev_div = 0;

        if self.len() < total.len() {
            self.resize(total.len(), 0);
        }

        for (i, x) in self.iter_mut().enumerate() {
            let subt = *x + total.get(i).unwrap_or(&0) + prev_div;
            let (div, modulo) = (subt / 10, subt % 10);

            *x = modulo;
            prev_div = div;
        }

        if prev_div > 0 {
            self.push(prev_div);
        }
    }
}

fn is_palindrome(digits: &Vec<u8>) -> bool {
    let len = digits.len() - 1;
    let end = digits.len() / 2;

    digits[0..end]
        .iter()
        .enumerate()
        .all(|(i, n)| *n == digits[len - i])
}

fn is_lychrel(n: u32) -> bool {
    let mut cycles = 50;
    let mut digits = to_digits(n);
    let mut is_lychrel = true;

    while cycles > 0 {
        let mut digits_rev = digits.clone();
        digits_rev.reverse();
        digits.sum_vec(&digits_rev);

        if is_palindrome(&digits) {
            is_lychrel = false;
            break;
        }

        cycles -= 1
    }
    is_lychrel
}

#[test]
fn test_is_lychrel() {
    assert_eq!(is_lychrel(47), false);
    assert_eq!(is_lychrel(196), true);
    assert_eq!(is_lychrel(349), false);
}

fn problem_55() -> usize {
    (1..10_000).filter(|&n| is_lychrel(n)).count()
}

#[test]
fn test_problem_55() {
    assert_eq!(problem_55(), 249)
}
