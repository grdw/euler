fn totient(bound: usize) -> usize {
    let mut n = 0;

    for i in 1..bound {
        if gcd(i, bound) == 1 {
            n += 1
        }
    }

    n
}

#[test]
fn test_totient() {
    assert_eq!(totient(2), 1);
    assert_eq!(totient(3), 2);
    assert_eq!(totient(4), 2);
    assert_eq!(totient(5), 4);
    assert_eq!(totient(6), 2);
    assert_eq!(totient(7), 6);
    assert_eq!(totient(8), 4);
    assert_eq!(totient(9), 6);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn max_totient(bound: usize) -> usize {
    let mut max = 0;

    for n in 1..=bound {
        let t = totient(n);
        if n % 10_000 == 0 {
            println!("---- {}", n);
        }
        if t > max {
            max = t;
        }
    }

    max
}

#[test]
fn test_max_totient() {
    assert_eq!(max_totient(10), 6);
}

fn main() {
    println!("answer: {}", max_totient(1_000_000));
}
