fn euler_totient(mut n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut result = n;
    // p is a prime factor
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

#[test]
fn test_euler_totient() {
    assert_eq!(euler_totient(5), 4);
}

fn max_totient(bound: usize) -> usize {
    let mut highest_pick: usize = 0;
    let mut max_totient: f32 = 0.0;

    for n in 2..=bound {
        let mut totient = euler_totient(n);
        let div = (n as f32) / (totient as f32);

        if div > max_totient {
            max_totient = div;
            highest_pick = n;
        }
    }

    highest_pick
}

#[test]
fn test_max_totient() {
    assert_eq!(max_totient(10), 6);
}

fn main() {
    println!("answer: {}", max_totient(1_000_000));
}
