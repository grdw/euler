fn main() {
    println!("Hello, world!");
}

const MAX_POWER: u32 = 100;

fn int_to_vec(mut number: u128) -> Vec<u8> {
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

#[test]
fn test_int_to_vec() {
    assert_eq!(int_to_vec(0), vec![]);
    assert_eq!(int_to_vec(16), vec![6, 1]);
    assert_eq!(int_to_vec(128), vec![8, 2, 1])
}

fn sum_arrays(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut divs = vec![];
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();
    let mut prev_mod = 0;

    loop {
        let total = match (a_iter.next(), b_iter.next()) {
            (Some(x), Some(y)) => *x + *y,
            (Some(x), None) => *x,
            (None, Some(y)) => *y,
            (None, None) => break
        };

        let (div, modulo) = (total % 10, total / 10);
        divs.push(div + prev_mod);
        prev_mod = modulo;
    }

    if prev_mod > 0 {
        divs.push(prev_mod);
    }

    divs
}

#[test]
fn test_summing_arrays() {
    // 0 + 80 = 80
    assert_eq!(sum_arrays(vec![], vec![0, 8]), vec![0, 8]);
    // 120 + 80 = 200
    assert_eq!(sum_arrays(vec![0, 2, 1], vec![0, 8]), vec![0, 0, 2]);
    // 92 + 80 = 271
    assert_eq!(sum_arrays(vec![2, 9], vec![0, 8]), vec![2, 7, 1]);
}

#[test]
fn test_summing_multiple_arrays() {
    let mut start = vec![1, 1, 1];
    start = sum_arrays(start, vec![1, 1, 2]);
    start = sum_arrays(start, vec![1, 1, 2]);

    assert_eq!(start, vec![3, 3, 5]);
}

fn multiply(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];

    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let mut total = vec![0; i + j];
            let mut mul_vec = int_to_vec((x * y) as u128);

            total.append(&mut mul_vec);
            result = sum_arrays(result, total);
        }
    }

    result
}

#[test]
fn test_multiply() {
    assert_eq!(
        multiply(
            int_to_vec(16),
            int_to_vec(28)
        ),
        vec![8, 4, 4]
    );

    assert_eq!(
        multiply(
            int_to_vec(28000),
            int_to_vec(1)
        ),
        vec![0, 0, 0, 8, 2]
    )
}

fn problem_16(power: u32) -> u16 {
    let mut result = vec![1];
    let cycles = power / MAX_POWER;
    let mut powers = vec![MAX_POWER; cycles as usize];
    let rest = power % MAX_POWER;

    if rest > 0 {
        powers.push(rest);
    }

    for p in &powers {
        result = multiply(result, int_to_vec(2_u128.pow(*p)));
    }

    result.iter().fold(0, |t, x| t + *x as u16)
}

#[test]
fn test_problem_16() {
    assert_eq!(problem_16(15), 26);
    assert_eq!(problem_16(115), 164);
    assert_eq!(problem_16(1000), 1366);
}
