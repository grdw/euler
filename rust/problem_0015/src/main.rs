fn main() {
    println!("Hello, sworld!");
}

fn problem_15(mut k: u128) -> u128 {
    let mut n = (k * 2) - 1;
    let mut o = 1;
    let mut p = 1;

    while k > 0 {
        o *= n;
        p *= k;
        n -= 1;
        k -= 1
    }

    (o / p) * 2
}


#[test]
fn test_problem_15() {
    assert_eq!(problem_15(2), 6);
    assert_eq!(problem_15(3), 20);
    assert_eq!(problem_15(20), 137846528820)
}
