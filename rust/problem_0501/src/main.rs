fn main() {
    println!("Hello, world!");
}

fn has_eight_divisors(num: u64) -> bool {
    let eight = 8;
    let pfactors = primes::factors(num);
    let divisors = primes::factors_uniq(num)
        .iter()
        .fold(1, |total, t| {
            let c = pfactors.iter().filter(|s| *s == t).count() + 1;
            total * c
        });

    divisors == eight
}

fn problem_501(n: u64) -> u64 {
    let mut t = 0;
    for j in 0..=n {
        if has_eight_divisors(j) {
            t += 1;
        }
    }
    t
}


#[test]
fn test_ranges() {
    assert_eq!(problem_501(100), 10);
    assert_eq!(problem_501(1000), 180);
    assert_eq!(problem_501(10_u64.pow(6)), 224427);
    assert_eq!(problem_501(10_u64.pow(12)), 224427)
}


#[test]
fn test_eight_divisors() {
    assert_eq!(has_eight_divisors(1), false);
    assert_eq!(has_eight_divisors(24), true);
    assert_eq!(has_eight_divisors(30), true);
    assert_eq!(has_eight_divisors(105), true);
    assert_eq!(has_eight_divisors(4738045415912923), false)
}
