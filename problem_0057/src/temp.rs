
#[derive(Debug, PartialEq)]
struct Fraction(i64, i64);

fn sqrt_of_2(n: usize) -> Vec<Fraction> {
    let mut list = vec![];

    for _ in 0..n {
        list.push(Fraction(2, 1))
    }

    list[0].0 = 1;
    return list
}

fn real(l: Vec<Fraction>) -> f64 {
    let mut t: f64 = 0.0;

    for i in (1..(l.len() - 1)).rev() {
        t = l[i].1 as f64 / (l[i].0 as f64 + t);
    }

    return t + l[0].0 as f64
}

#[test]
fn test_expand_sqrt_of_2() {
    assert_eq!(real(sqrt_of_2(20)), 1.414213562373087);
    assert_eq!(real(sqrt_of_2(4)), 1.4);
    assert_eq!(real(sqrt_of_2(3)), 1.5);
    assert_eq!(real(sqrt_of_2(2)), 1.0);
    assert_eq!(real(sqrt_of_2(1)), 1.0);
}
