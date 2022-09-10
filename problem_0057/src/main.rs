use std::cmp;

fn main() {
    let result = sqrt_of_2(1000);
    println!("The answer to euler#57 is {}", result)
}

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

pub trait VecEx<U8> {
    fn sum_vec(&self, addition: &Vec<u8>) -> Vec<u8>;
}

impl VecEx<u8> for Vec<u8> {
    fn sum_vec(&self, addition: &Vec<u8>) -> Vec<u8> {
        let mut result = vec![];
        let mut prev_div = 0;
        let max = cmp::max(self.len(), addition.len());

        for i in 0..max {
            let subt = self.get(i).unwrap_or(&0) +
                       addition.get(i).unwrap_or(&0) +
                       prev_div;
            let (div, modulo) = (subt / 10, subt % 10);

            result.push(modulo);
            prev_div = div;
        }

        if prev_div > 0 {
            result.push(prev_div);
        }

        result
    }
}

#[test]
fn test_sum_vec() {
    let n = 1500.to_vec();
    let t = 12.to_vec();

    let q = n.sum_vec(&t);
    assert_eq!(q, vec![2, 1, 5, 1]);

    let n = 1500.to_vec();
    let t = 12.to_vec();

    let q = t.sum_vec(&n);
    assert_eq!(q, vec![2, 1, 5, 1]);

    let n = 239.to_vec();
    let t = 169.to_vec();

    let q = t.sum_vec(&n);
    assert_eq!(q, vec![8, 0, 4]);
}

fn sqrt_of_2(n: u128) -> u128 {
    let mut counts = 0;
    let mut prev_den;
    let mut num = 3.to_vec();
    let mut den = 2.to_vec();

    for _ in 1..n {
        prev_den = den.clone();
        den = den.sum_vec(&num);
        num = den.sum_vec(&prev_den);

        if num.len() > den.len() {
            counts += 1;
        }
    }

    counts
}

#[test]
fn test_expand_sqrt_of_2() {
    assert_eq!(sqrt_of_2(7), 0);
    assert_eq!(sqrt_of_2(8), 1);
    assert_eq!(sqrt_of_2(1000), 153);
}
