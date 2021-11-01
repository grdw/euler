fn main() {
    println!("Hello, world!");
}

fn range_pattern_count(range: &Vec<u16>, m: u16) -> Option<u16> {
    if range.len() == 0 {
        return None;
    }

    let mut count = 0;
    let mut iter = range.iter().rev();

    loop {
        count += 1;

        let val = iter.next().unwrap_or(&0);
        if *val == m {
            break Some(count)
        } else if *val == 0 {
            break None
        }
    }
}

#[test]
fn test_range_pattern_count() {
    assert_eq!(range_pattern_count(&vec![], 1), None);
    assert_eq!(range_pattern_count(&vec![1], 1), Some(1));
    assert_eq!(range_pattern_count(&vec![5, 2], 2), Some(1));
    assert_eq!(range_pattern_count(&vec![5, 10, 1, 2], 2), Some(1));
    assert_eq!(range_pattern_count(&vec![1, 2, 3], 1), Some(3));
    assert_eq!(range_pattern_count(&vec![1, 2, 3], 4), None);
}

fn cycle_count(n: u16, d: u16) -> u16 {
    let base = 10;
    let mut m = n;
    let mut range = vec![];

    loop {
        m = m * base % d;

        if m == 0 {
            break 0;
        }

        if let Some(rpc) = range_pattern_count(&range, m) {
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

fn problem_26() -> u16 {
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
