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

fn problem_30() -> u128 {
    let mut matches = vec![];
    for n in 2..=999999 {
        let result: u128 = n.to_vec()
            .iter()
            .map(|d| (*d as u128).pow(5))
            .fold(0, |dp, acc| acc + dp);

        if n == result {
            matches.push(n);
        }
    }
    matches.iter().fold(0, |p, acc| *acc + p)
}

#[test]
fn test_problem_30_brute_force() {
    assert_eq!(problem_30(), 443839);
}

fn fifth_power_sum(nums: &Vec<u32>) -> u32 {
    nums
        .iter()
        .enumerate()
        .map(|(i, b)| (i as u32 + 1).pow(5) * b)
        .sum()
}

#[test]
fn test_fifth_power_sum() {
    assert_eq!(fifth_power_sum(&vec![1, 0, 0, 1, 5, 0, 0, 0, 0]), 16650);
    assert_eq!(fifth_power_sum(&vec![1, 0, 0, 1, 1, 0, 0, 0, 0]), 4150);
    assert_eq!(fifth_power_sum(&vec![2, 0, 0, 1, 1, 0, 0, 0, 0]), 4151);
    assert_eq!(fifth_power_sum(&vec![0, 0, 1, 1, 0, 0, 0, 1, 1]), 93084);
    assert_eq!(fifth_power_sum(&vec![0, 2, 0, 0, 0, 0, 2, 0, 1]), 92727);
    assert_eq!(fifth_power_sum(&vec![1, 0, 0, 1, 0, 0, 1, 0, 3]), 194979);
}

fn reverse_number_system(num: u128) -> Vec<u32> {
    let l = num.to_vec();
    let mut result = vec![0;9];

    for n in &l {
        if *n == 0 { continue }

        result[*n as usize - 1] += 1
    }

    result
}

#[test]
fn test_reverse_number_system() {
    assert_eq!(
        reverse_number_system(16650),
        vec![1, 0, 0, 0, 1, 2, 0, 0, 0]
    );
    assert_eq!(
        reverse_number_system(194979),
        vec![1, 0, 0, 1, 0, 0, 1, 0, 3]
    );
}

fn problem_30_improved() -> u32 {
    let mut start:Vec<u32> = vec![0; 9];
    let mut index: usize = 0;
    let mut sum = 0;

    while index < start.len() {
        if start[index] > 2 {
            start[index] = 0;
            index += 1;
        } else {
            start[index] += 1;
            if start[index] <= index as u32 {
                index = 0;
            }

            let t = fifth_power_sum(&start);
            let s = reverse_number_system(t as u128);

            if s == start && t > 1 {
                sum += t
            }
        }
    }

    sum
}

#[test]
fn test_problem_30_improved() {
    assert_eq!(problem_30_improved(), 443839);
}
