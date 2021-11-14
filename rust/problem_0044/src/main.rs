fn is_pentagonal(i: u64) -> bool {
    let n = ((((24 * i) + 1) as f64).sqrt() + 1.0) / 6.0;

    n.fract() == 0.0
}

#[test]
fn test_is_pentagonal() {
    assert_eq!(is_pentagonal(12), true);
    assert_eq!(is_pentagonal(9), false);
}

fn to_pentagonal_number(i: u64) -> u64 {
    (i * (3 * i - 1)) / 2
}

#[test]
fn test_to_pentagonal_number() {
    assert_eq!(to_pentagonal_number(4), 22);
    assert_eq!(to_pentagonal_number(12), 210);
}

fn problem_44() -> u64 {
    let j = 1;
    let mut k = 1;

    loop {
        k += 1;

        let mut m: u64 = 0;
        let pk = to_pentagonal_number(k);

        for l in j..k {
            let pl = to_pentagonal_number(l);

            if is_pentagonal(pk + pl) && is_pentagonal(pk - pl) {
                m = pk - pl;
                break;
            }
        }

        if m > 0 {
            break m
        }
    }
}

#[test]
fn test_problem_44() {
    assert_eq!(problem_44(), 5482660);
}
