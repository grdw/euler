fn cancelled_simplified(
    num: f64,
    den: f64,
    simple_num: f64,
    simple_den: f64) -> bool {

    let full_s = format!("{}{}", num, den);
    let small_s = format!("{}{}", simple_num, simple_den);
    let all_present = small_s.chars().all(|x| full_s.contains(x));

    let mut index = 0;
    for f in small_s.chars() {
        let m = full_s.find(f);

        if m.is_some() {
            index += m.unwrap();
        }
    }

    all_present && index == 3 &&
        (full_s.chars().nth(1).unwrap() == full_s.chars().nth(2).unwrap())
}

#[test]
fn test_cancelled_simplified() {
    assert_eq!(cancelled_simplified(49.0, 98.0, 4.0, 8.0), true);
    assert_eq!(cancelled_simplified(49.0, 98.0, 1.0, 2.0), false);
    assert_eq!(cancelled_simplified(30.0, 50.0, 3.0, 5.0), false);
    assert_eq!(cancelled_simplified(80.0, 96.0, 5.0, 6.0), false);
    assert_eq!(cancelled_simplified(72.0, 96.0, 9.0, 12.0), false);
    assert_eq!(cancelled_simplified(46.0, 92.0, 2.0, 4.0), false);
    assert_eq!(cancelled_simplified(15.0, 50.0, 3.0, 10.0), false);
    assert_eq!(cancelled_simplified(24.0, 42.0, 4.0, 7.0), false);
    assert_eq!(cancelled_simplified(12.0, 24.0, 1.0, 2.0), false);
}

fn simplify_fraction(num: f64, den: f64) -> Vec<u16> {
    let factor = den / num;
    let mut n = den.sqrt() as u16;
    let mut result = vec![];

    while n > 0 {
        let div_num = n as f64;
        let div_den = n as f64 * factor;
        let is_zero = (div_num.fract() + div_den.fract()) == 0.0;

        if is_zero && cancelled_simplified(num, den, div_num, div_den) {
            result = vec![
                div_num as u16,
                div_den as u16
            ];
        }
        n -= 1;
    }

    result
}

#[test]
fn test_simplify_fraction() {
    assert_eq!(simplify_fraction(49.0, 98.0), vec![4, 8]);
    assert_eq!(simplify_fraction(6.0, 9.0), vec![]);
    assert_eq!(simplify_fraction(60.0, 90.0), vec![]);
}

fn problem_33() -> u16 {
    let mut total_n = 1;
    let mut total_d = 1;

    for num in 10..=99 {
        for den in 10..=99 {
            let num_f: f64 = num as f64;
            let den_f: f64 = den as f64;

            if (num_f / den_f) < 1.0 {
                let simplified = simplify_fraction(num_f, den_f);

                if !simplified.is_empty() {
                    total_n *= simplified[0];
                    total_d *= simplified[1];
                }
            }
        }
    }
    total_d / total_n
}

#[test]
fn test_problem_33() {
    assert_eq!(problem_33(), 100);
}

