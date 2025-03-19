fn main() {
    let answer = problem_64();
    println!("The answer to p64 = {}", answer);
}

fn odd_pattern(x: u64) -> bool {
    let a0: f64 = (x as f64).sqrt();

    if (a0 - a0.floor()).abs() < ::std::f64::EPSILON {
        return false
    }

    let mut m: u64 = 0;
    let mut d: u64 = 1;
    let mut a: u64 = a0.floor() as u64;
    let mut expansion = vec![];
    let end: u64 = a * 2;

    while a != end {
        m = d * a - m;
        d = (x - m * m) / d;
        a = ((a0 + m as f64) / d as f64).floor() as u64;
        expansion.push(a);
    }

    expansion.len() % 2 != 0
}

#[test]
fn test_examples_odd_pattern() {
    assert_eq!(odd_pattern(2), true);
    assert_eq!(odd_pattern(3), false);
    assert_eq!(odd_pattern(5), true);
    assert_eq!(odd_pattern(6), false);
    assert_eq!(odd_pattern(7), false);
    assert_eq!(odd_pattern(8), false);
    assert_eq!(odd_pattern(9), false);
    assert_eq!(odd_pattern(10), true);
    assert_eq!(odd_pattern(11), false);
    assert_eq!(odd_pattern(12), false);
    assert_eq!(odd_pattern(13), true);

    let n = (0..=13).filter(|x| odd_pattern(*x)).count();
    assert_eq!(n, 4);
}

#[test]
fn test_debug_odd_pattern() {
    assert_eq!(odd_pattern(329), false);
}

fn problem_64() -> usize {
    (0..=10_000).filter(|x| odd_pattern(*x)).count()
}

#[test]
fn test_problem_64() {
    assert_eq!(problem_64(), 1322);
}
