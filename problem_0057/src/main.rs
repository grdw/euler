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
    fn sum_vec(&mut self, total: &Vec<u8>);
    fn min_vec(&self, total: &Vec<u8>) -> Vec<u8>;
    fn multiply(&mut self, total: &Vec<u8>);
}

impl VecEx<u8> for Vec<u8> {
    fn sum_vec(&mut self, total: &Vec<u8>) {
        let mut prev_div = 0;

        if self.len() < total.len() {
            self.resize(total.len(), 0);
        }

        for (i, x) in self.iter_mut().enumerate() {
            let subt = *x + total.get(i).unwrap_or(&0);
            let (div, modulo) = (subt / 10, subt % 10);

            *x = modulo + prev_div;
            prev_div = div;
        }

        if prev_div > 0 {
            self.push(prev_div);
        }
    }

    fn min_vec(&self, sub: &Vec<u8>) -> Vec<u8> {
        let mut result = vec![];
        let mut prev_sub = 0;

        for (i, x) in self.iter().enumerate() {
            let val = sub.get(i).unwrap_or(&0);
            let mut div = *x - prev_sub;

            if val > &div {
                prev_sub = 1;
                div += 10;
            } else {
                prev_sub = 0;
            }

            result.push(div - val);
        }

        // Drops off any zeroes
        for i in (0..result.len()).rev() {
            if result[i] == 0 {
                result.pop();
            } else {
                break;
            }
        }

        result
    }

    fn multiply(&mut self, m: &Vec<u8>) {
        let mut totals = vec![];

        for (i, x) in self.iter().enumerate() {
            for (j, y) in m.iter().enumerate() {
                let mut total = vec![0; i + j];
                let mut mul_vec = ((x * y) as u128).to_vec();

                total.append(&mut mul_vec);
                totals.push(total);
            }
        }

        self.clear();
        for t in totals {
            self.sum_vec(&t)
        }
    }
}

#[test]
fn test_sum_vec() {
    let mut ten = 10.to_vec();
    let two = 2.to_vec();

    ten.sum_vec(&two);
    assert_eq!(ten, vec![2, 1]);
}

#[test]
fn test_mul_vec() {
    let mut ten = 10.to_vec();
    let two = 2.to_vec();

    ten.multiply(&two);
    assert_eq!(ten, vec![0, 2]);
}

#[test]
fn test_min_vec() {
    let ten = 10.to_vec();
    let two = 2.to_vec();

    let result = ten.min_vec(&two);
    assert_eq!(result, vec![8]);

    let x = 1500.to_vec();
    let y = 200.to_vec();
    let result = x.min_vec(&y);

    assert_eq!(result, vec![0, 0, 3, 1]);

    let x = 1500.to_vec();
    let y = 1500.to_vec();
    let result = x.min_vec(&y);

    assert_eq!(result, vec![]);

    let x = 7432.to_vec();
    let y = 1322.to_vec();
    let result = x.min_vec(&y);

    assert_eq!(result, vec![0, 1, 1, 6]);
}

#[test]
fn test_min_vec_hard() {
    let x = 1947827.to_vec();
    let y = 973814.to_vec();
    let result = x.min_vec(&y);

    assert_eq!(result, vec![3, 1, 0, 4, 7, 9]);
}

fn main() {
    println!("Hello, world!");

}

fn sqrt_of_2(n: u128) -> u128 {
    let mut c = 0;
    let mut den = 2.to_vec();
    let mut num = 3.to_vec();

    for i in 0..n {
        let q = 2.to_vec();
        den.multiply(&q);
        num.sum_vec(&den);
        den = num.min_vec(&den);

        if num.len() > den.len() {
            c+=1;
        }
    }

    c
}

#[test]
fn test_expand_sqrt_of_2() {
    assert_eq!(sqrt_of_2(1000), 301);
    assert_eq!(sqrt_of_2(8), 1);
    assert_eq!(sqrt_of_2(4), 0);
    assert_eq!(sqrt_of_2(3), 0);
    assert_eq!(sqrt_of_2(2), 0);
    assert_eq!(sqrt_of_2(1), 0);
}
