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
    fn sum_vec(&mut self, total: Vec<u8>);
    fn multiply(&mut self, total: Vec<u8>);
}

impl VecEx<u8> for Vec<u8> {
    fn sum_vec(&mut self, total: Vec<u8>) {
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

    fn multiply(&mut self, m: Vec<u8>) {
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
            self.sum_vec(t)
        }
    }
}

fn problem_20(factor: u128) -> u64 {
    let mut start = vec![1];
    for n in 2..=factor {
        let result = n.to_vec();
        start.multiply(result)
    }
    start.iter().map(|&u| u as u64).sum()
}

#[test]
fn test_problem_20() {
    assert_eq!(problem_20(10), 27);
    assert_eq!(problem_20(100), 648);
}
