fn main() {
    println!("Hello, world!");
}

fn sum_divisors(i: u16) -> u16 {
    let sqrt = (i as f64).sqrt() as u16;
    let mut total_div = 1;
    for n in 2..sqrt {
        if i % n == 0 {
            total_div += n + (i / n);
        }
    }
    total_div
}

#[test]
fn test_divisors() {
    assert_eq!(sum_divisors(220), 284);
    assert_eq!(sum_divisors(60), 108);
    assert_eq!(sum_divisors(sum_divisors(220)), 220);
}

fn amicable(i: u16) -> bool {
    let n = sum_divisors(i);

    n != i && sum_divisors(n) == i
}

#[test]
fn test_amicable() {
    assert_eq!(amicable(1), false);
    assert_eq!(amicable(220), true);
}

fn problem_21() -> u16 {
    (0..10_000)
        .filter(|&n| amicable(n))
        .sum()
}

#[test]
fn test_problem_21() {
    assert_eq!(problem_21(), 31626);
}
