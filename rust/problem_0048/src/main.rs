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
    fn power_vec(&mut self, power: u16);
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

    fn power_vec(&mut self, power: u16) {
        let max = 11;
        let t = self.clone();
        let l = self.len();

        for _ in 1..power {
            let mut subtotals = vec![];

            'outer: for (i, x) in self.iter().enumerate() {
                for j in 0..l {
                    let y = t[j];
                    let mut total = vec![0; i + j];
                    let mut mul_vec = ((x * y) as u128).to_vec();

                    total.append(&mut mul_vec);
                    subtotals.push(total);

                    if i + j >= max {
                        break 'outer;
                    }
                }
            }

            self.clear();
            for subtotal in subtotals {
                self.sum_vec(&subtotal)
            }
        }
    }
}

#[test]
fn test_vector_methods() {
    let mut five = 5.to_vec();
    five.power_vec(5);

    assert_eq!(five, vec![5,2,1,3]);

    let mut t = 999.to_vec();
    t.power_vec(999);

    assert_eq!(t.len(), 13);
    assert_eq!(t, vec![9, 9, 9, 8, 9, 9, 9, 9, 4, 0, 0, 5, 1]);
}

fn problem_48() -> u64 {
    let mut start: u128 = 0;
    let mut total = vec![];

    while start < 1000 {
        start += 1;

        let mut subtotal = start.to_vec();
        subtotal.power_vec(start as u16);
        total.sum_vec(&subtotal);
    }

    total.truncate(10);

    total
        .iter()
        .enumerate()
        .fold(0, |acc, (i, t)| acc + 10_u64.pow(i as u32) * *t as u64)
}

#[test]
fn test_problem_48() {
    assert_eq!(problem_48(), 9110846700);
}
