pub trait ToVector {
    fn to_digits(&self) -> Vec<u8>;
}

impl ToVector for u128 {
    fn to_digits(&self) -> Vec<u8> {
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

pub trait VecEx<U8> {
    fn sum_vec(&mut self, total: &Vec<u8>);
    fn mul_vec(&mut self, total: &Vec<u8>);
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

    fn mul_vec(&mut self, m: &Vec<u8>) {
        let mut totals = vec![];

        for (i, x) in self.iter().enumerate() {
            for (j, y) in m.iter().enumerate() {
                let mut total = vec![0; i + j];
                let mut mul_vec = ((x * y) as u128).to_digits();

                total.append(&mut mul_vec);
                totals.push(total);
            }
        }

        self.clear();
        for t in &totals {
            self.sum_vec(t)
        }
    }
}

#[test]
fn test_sum_vec() {
    let mut d1 = 8100.to_digits();
    let mut d2 = 810.to_digits();

    d1.sum_vec(&mut d2);

    assert_eq!(d1, vec![0, 1, 9, 8]);

    let mut d1 = vec![];
    let mut d2 = 9.to_digits();

    d1.sum_vec(&mut d2);

    assert_eq!(d1, vec![9]);
}

#[test]
fn test_mul_vec() {
    let mut d = 9.to_digits();
    d.mul_vec(&9.to_digits());
    assert_eq!(d, vec![1, 8]);
    d.mul_vec(&9.to_digits());
    assert_eq!(d, vec![9, 2, 7]);

    let mut d2 = 99.to_digits();
    d2.mul_vec(&99.to_digits());
    assert_eq!(d2, vec![1, 0, 8, 9]);
}

fn max_power_digit_sum(digit: u128, power: u16) -> u64 {
    let mul_digits = digit.to_digits();
    let mut digits = digit.to_digits();
    let mut max = 0;

    for _ in 2..=power {
        digits.mul_vec(&mul_digits);

        let sum = digits.iter().fold(0, |acc, &n| acc + n as u64);

        if sum > max {
            max = sum;
        }
    }

    max
}

#[test]
fn test_max_power_digit_sum() {
    assert_eq!(max_power_digit_sum(10, 100), 1);
    assert_eq!(max_power_digit_sum(99, 100), 972);
}

fn problem_56() -> u64 {
    let mut max = 0;
    let mut a = 100;

    while a > 2 {
        let pds = max_power_digit_sum(a, 100);
        if pds > max {
            max = pds
        }
        a -= 1;
    }

    max
}

#[test]
fn test_problem_56() {
    assert_eq!(problem_56(), 972);
}
