fn is_divisible(digits: &Vec<char>) -> bool {
    let group_size = 2;
    let max = digits.len() - group_size;
    let primes = vec![2, 3, 5, 7, 11, 13, 17];
    let mut i = 1;

    loop {
        let d = &digits[i..=i + group_size];
        let n: u64 = d
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        if n % primes[i - 1] != 0 {
            break false
        }

        i += 1;

        if i >= max {
            break true
        }
    }
}

#[test]
fn test_is_divisible() {
    let g1 = vec!['1','4','0','6','3','5','7','2','8','9'];
    let g2 = vec!['4','0','1','6','3','5','7','2','8','9'];
    let g3 = vec!['4','0','1','7','3','5','6','2','8','9'];

    assert_eq!(is_divisible(&g1), true);
    assert_eq!(is_divisible(&g2), false);
    assert_eq!(is_divisible(&g3), false)
}

fn fits_div_rule_2_and_5(digits: &Vec<char>) -> bool {
    let rule_two = digits[3].to_digit(10).unwrap();
    let rule_five = digits[5];

    rule_two % 2 == 0 && (rule_five == '5' || rule_five == '0')
}

#[test]
fn test_fits_div_rule_2_and_5() {
    let g1 = vec!['0','0','0','2','0','5','0','0','0','0'];
    let g2 = vec!['0','0','0','3','0','5','0','0','0','0'];
    let g3 = vec!['0','0','0','2','0','6','0','0','0','0'];

    assert_eq!(fits_div_rule_2_and_5(&g1), true);
    assert_eq!(fits_div_rule_2_and_5(&g2), false);
    assert_eq!(fits_div_rule_2_and_5(&g3), false)
}

fn problem_43() -> u64 {
    let mut sum: u64 = 0;
    let mut digits = vec![
        '9', '8', '7', '6', '5', '4', '3', '2', '1', '0'
    ];

    let mut result: Vec<usize> = vec![0; digits.len()];
    let mut i = 0;

    while i < digits.len() {
        if result[i] < i {
            if i % 2 == 0 {
                digits.swap(0, i);
            } else {
                digits.swap(result[i], i);
            }

            if fits_div_rule_2_and_5(&digits) && is_divisible(&digits) {
                let n: u64 = digits
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                sum += n;
            }
            result[i] += 1;
            i = 0;
        } else {
            result[i] = 0;
            i += 1
        }
    }

    sum
}

#[test]
fn test_problem_43() {
    assert_eq!(problem_43(), 16695334890)
}
