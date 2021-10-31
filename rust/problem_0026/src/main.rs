fn main() {
    println!("Hello, world!");
}

fn division(n: u128, d: u128) -> u64 {
    let base = 10;
    let mut m = n;
    let mut range = vec![];

    loop {
        m = m * base % d;

        if m == 0 {
            break 0;
        }

        if range.len() > 0 {
            if range.contains(&m) {
                break range.len() as u64;
            }
        }

        range.push(m);
    }
}

#[test]
fn test_division_below_10() {
    assert_eq!(division(1, 2), 0);
    assert_eq!(division(1, 3), 1);
    assert_eq!(division(1, 4), 0);
    assert_eq!(division(1, 5), 0);
    assert_eq!(division(1, 6), 1);
    assert_eq!(division(1, 7), 6);
    assert_eq!(division(1, 8), 0);
    assert_eq!(division(1, 9), 1);
    assert_eq!(division(1, 10), 0);
}

#[test]
fn test_division_more_than_10() {
    // The interesting numbers:
    assert_eq!(division(1, 11), 2);
    assert_eq!(division(1, 12), 2);
    assert_eq!(division(1, 15), 1);
    assert_eq!(division(1, 97), 96);
    assert_eq!(division(1, 983), 982);
    assert_eq!(division(1, 60), 2);
}

fn problem_26() -> u128 {
    let mut resulting_cycle_count = 0;
    let mut number = 0;
    for n in 1..1000 {
        let cycle_count = division(1, n);
        if cycle_count > resulting_cycle_count {
            resulting_cycle_count = cycle_count;
            number = n;
        }
    }
    number
}

#[test]
fn test_problem_26() {
    assert_eq!(problem_26(), 983);
}
