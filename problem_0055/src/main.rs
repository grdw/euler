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
        let mut remainder = 0;
        let mut len = self.len();

        while len > 0 {
            let i = len - 1;
            let subt = self[i] + total[i] + remainder;
            let (div, modulo) = (subt / 10, subt % 10);

            self[i] = modulo;
            remainder = div;
            len -= 1;
        }

        if remainder > 0 {
            self.insert(0, remainder);
        }
    }
}

#[test]
fn test_sum_vec() {
    let mut d = vec![1, 5, 8, 8];
    d.sum_vec(&vec![9, 9, 9, 9]);
    assert_eq!(d, vec![1, 1, 5, 8, 7]);

    let mut d2 = vec![4, 7];
    d2.sum_vec(&vec![7, 4]);
    assert_eq!(d2, vec![1, 2, 1]);
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

        println!("{:?}", digits);
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
