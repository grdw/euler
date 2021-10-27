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

fn sum_array(result: &mut Vec<u8>, total: Vec<u8>) {
    let mut prev_div = 0;

    if result.len() < total.len() {
        result.resize(total.len(), 0);
    }

    for (i, x) in result.iter_mut().enumerate() {
        let subt = *x + total.get(i).unwrap_or(&0);
        let (div, modulo) = (subt / 10, subt % 10);

        *x = modulo + prev_div;
        prev_div = div;
    }

    if prev_div > 0 {
        result.push(prev_div);
    }
}

#[test]
fn test_summing_arrays() {
    let mut result = vec![];
    sum_array(&mut result, vec![0, 8]);
    assert_eq!(result, vec![0, 8]);

    let mut result2 = vec![0, 2, 1];
    sum_array(&mut result2, vec![0, 8]);
    assert_eq!(result2, vec![0, 0, 2]);

    let mut result3 = vec![2, 9];
    sum_array(&mut result3, vec![0, 8]);
    assert_eq!(result3, vec![2, 7, 1])
}

fn multiply(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];

    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let mut total = vec![0; i + j];
            let mut mul_vec = int_to_vec((x * y) as u128);
            total.append(&mut mul_vec);

            sum_array(&mut result, total)
        }
    }

    result
}

#[test]
fn test_multiply() {
    assert_eq!(
        multiply(int_to_vec(16), int_to_vec(28)),
        vec![8, 4, 4]
    );

    assert_eq!(
        multiply(int_to_vec(5), int_to_vec(6)),
        vec![0, 3]
    );

    assert_eq!(
        multiply(int_to_vec(28000), int_to_vec(1)),
        vec![0, 0, 0, 8, 2]
    )
}

fn problem_16(power: u32) -> u16 {
    let (cycles, rest) = (power / MAX_POWER, power % MAX_POWER);
    let mut result = vec![1];
    let mut powers = vec![MAX_POWER; cycles as usize];

    if rest > 0 {
        powers.push(rest);
    }

    for p in &powers {
        let parr = int_to_vec(2_u128.pow(*p));

        result = multiply(result, parr);
    }

    result.iter().map(|&n| n as u16).sum()
}

#[test]
fn test_problem_16() {
    assert_eq!(problem_16(15), 26);
    assert_eq!(problem_16(115), 164);
    assert_eq!(problem_16(1000), 1366);
}
