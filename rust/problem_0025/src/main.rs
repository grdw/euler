fn main() {
    println!("Hello, world!");
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

#[test]
fn test_sum_vec() {
    let mut start = vec![9,9,9,9];
    let add = vec![1];
    start.sum_vec(&add);
    assert_eq!(start, vec![0,0,0,0,1]);

    let mut start = vec![9,9,9,8];
    let add = vec![1];
    start.sum_vec(&add);
    assert_eq!(start, vec![0,0,0,9]);

    let mut start = vec![9,9,9,9,9,9,9,9];
    let add = vec![9,9,9,9,9,9,9,9];
    start.sum_vec(&add);
    // 99_999_999 + 99_999_999 = 199_999_998
    assert_eq!(start, vec![8, 9, 9, 9, 9, 9, 9, 9, 1]);
}

fn fibonacci_thousand(start: u128, max: usize) -> u128 {
    let mut far = 1.to_vec();
    let mut addition = start.to_vec();
    let mut index = start;

    loop {
        far.sum_vec(&addition);
        addition.sum_vec(&far);
        index += 2;

        if far.len() >= max {
            break index;
        }
    }
}

fn problem_25(max: usize) -> u128 {
    let i = fibonacci_thousand(2, max);
    let k = fibonacci_thousand(1, max);

    if i > k {
        k
    } else {
        i
    }
}

#[test]
fn test_fibonacci_thousand() {
    assert_eq!(problem_25(2), 7);
}

#[test]
fn test_problem_25() {
    assert_eq!(problem_25(1000), 4782);
}
