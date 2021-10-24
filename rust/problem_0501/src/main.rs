use primes::{Sieve, PrimeSet};

fn main() {
    println!("Hello, world!");
}

fn problem_501(n: u64) -> u64 {
    let mut t = 0;
    let mut pset = Sieve::new();

    // When three unique primes
    for p in pset.iter() {
        if p > 2 { break }

        let mut pset = Sieve::new();

        for p2 in pset.iter() {
            if p2 > 5 { break }
            if p >= p2 { continue }

            let mut pset = Sieve::new();

            for p3 in pset.iter() {
                if p3 > 13 { break }
                if p2 >= p3 || p >= p3 { continue }

                if p * p2 * p3 < n {
                    t += 1;
                }
            }
        }
    }

    // When one prime to the power of 3 with one other prime
    for p in pset.iter() {
        let o = p.pow(3);

        if p > n || o > n { break }

        let mut pset = Sieve::new();

        for p2 in pset.iter() {
            let o = o * p2;

            if p2 > n || o > n { break }
            if p == p2 { continue }

            t += 1
        }
    }

    // When one other prime is the power of 7
    for p in pset.iter() {
        let o = p.pow(7);

        if o > n { break }
        if o < n { t += 1 }
    }


    t
}


#[test]
fn test_ranges() {
    assert_eq!(problem_501(100), 10);
    //assert_eq!(problem_501(1000), 180);
    //assert_eq!(problem_501(10_u64.pow(6)), 224427);
    //assert_eq!(problem_501(10_u64.pow(12)), 224427)
}


#[test]
fn test_eight_divisors() {
    //assert_eq!(has_eight_divisors(1), false);
    //assert_eq!(has_eight_divisors(24), true);
    //assert_eq!(has_eight_divisors(30), true);
    //assert_eq!(has_eight_divisors(105), true);
    //assert_eq!(has_eight_divisors(999), true);
    //assert_eq!(has_eight_divisors(4738045415912923), false);
    //assert_eq!(has_eight_divisors(1_000_000_000_000_000), false)
}
