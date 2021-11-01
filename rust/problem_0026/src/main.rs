fn main() {
    println!("Hello, world!");
}

fn range_pattern_count(range: &Vec<u128>, m: u128) -> u64 {
    if range.len() == 0 {
        return 0;
    }

    if range[range.len() - 1] == m {
        1
    } else if range.contains(&m) {
        range.len() as u64
    } else {
        0
    }
}

#[test]
fn test_range_pattern_count() {
    assert_eq!(range_pattern_count(&vec![], 1), 0);
    assert_eq!(range_pattern_count(&vec![1], 1), 1);
    assert_eq!(range_pattern_count(&vec![5, 2], 2), 1);
    assert_eq!(range_pattern_count(&vec![1, 2, 3], 1), 3);
    assert_eq!(range_pattern_count(&vec![1, 2, 3], 4), 0);
}

fn cycle_count(n: u128, d: u128) -> u64 {
    let base = 10;
    let mut m = n;
    let mut range = vec![];

    loop {
        m = m * base % d;

        if m == 0 {
            break 0;
        }

        let rpc = range_pattern_count(&range, m);
        if rpc > 0 {
            break rpc;
        }

        range.push(m);
    }
}

#[test]
fn test_cycle_count_below_10() {
    assert_eq!(cycle_count(1, 2), 0);
    assert_eq!(cycle_count(1, 3), 1);
    assert_eq!(cycle_count(1, 4), 0);
    assert_eq!(cycle_count(1, 5), 0);
    assert_eq!(cycle_count(1, 6), 1);
    assert_eq!(cycle_count(1, 7), 6);
    assert_eq!(cycle_count(1, 8), 0);
    assert_eq!(cycle_count(1, 9), 1);
    assert_eq!(cycle_count(1, 10), 0);
}

#[test]
fn test_cycle_count_more_than_10() {
    // The interesting numbers:
    assert_eq!(cycle_count(1, 11), 2);
    assert_eq!(cycle_count(1, 12), 1);
    assert_eq!(cycle_count(1, 15), 1);
    assert_eq!(cycle_count(1, 97), 96);
    assert_eq!(cycle_count(1, 983), 982);
    assert_eq!(cycle_count(1, 60), 1);
}

fn problem_26() -> u128 {
    let mut resulting_cycle_count = 0;
    let mut number = 0;
    for n in 1..1000 {
        let cycle_count = cycle_count(1, n);
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
