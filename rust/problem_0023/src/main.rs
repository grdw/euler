fn main() {
    println!("Hello, world!");
}

fn sum_proper_divisors(i: u32) -> u32 {
    let sqrt = (i as f64).sqrt() as u32;
    let mut total_div = 1;
    for n in 2..=sqrt {
        if i % n == 0 {
            let total = if n == i / n {
                n
            } else {
                n + (i / n)
            };
            total_div += total;
        }
    }
    total_div
}

fn abundant(i: u32) -> bool {
    sum_proper_divisors(i) > i
}

#[test]
fn test_abundant() {
    assert_eq!(abundant(12), true);
    assert_eq!(abundant(16), false);
    assert_eq!(abundant(28), false);
}

const MAX:u32 = 28123;

fn non_double_abundant(start: u32) -> u32 {
    let mut filter_set = vec![];
    let mut total = 0;
    let mut n = start;

    while n <= MAX {
        let mut double_abundant = false;

        for f in &filter_set {
            if abundant(n - f) {
                double_abundant = true;
                break;
            }
        }

        if abundant(n) {
            filter_set.push(n);
        }

        if !double_abundant {
            total += n;
        }

        n += 2;
    }
    total
}

fn problem_23() -> u32 {
    non_double_abundant(1) + non_double_abundant(2)
}

#[test]
fn test_problem_23() {
    assert_eq!(problem_23(), 4179871);
}

