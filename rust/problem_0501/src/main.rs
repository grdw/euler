use primes::{Sieve, PrimeSet};

fn main() {
    println!("Hello, world!");
}

fn problem_501(n: u64) -> u64 {
    let mut t = 0;
    let mut pset = Sieve::new();

    // Memory problem:
    //
    // This vector is going to be enormous when doing this.
    let primes: Vec<(usize, u64)> = pset
        .iter()
        .enumerate()
        .take(n as usize)
        .collect();

    // When three unique primes
    for (_, p) in &primes {
        if *p > n { break }

        for (_, p2) in &primes {
            let o = p * p2;

            if *p2 > n || o > n { break }
            if p >= p2 { continue }

            for (_, p3) in &primes {
                let o = o * p3;

                if *p3 > n || o > n { break }
                if p2 >= p3 || p >= p3 { continue }

                t += 1;
            }
        }
    }

    // When one prime to the power of 3 with one other prime
    for (_, p) in &primes {
        let o = p.pow(3);

        if *p > n || o > n { break }

        for (_, p2) in &primes {
            let o = o * p2;

            if *p2 > n || o > n { break }
            if p == p2 { continue }

            t += 1
        }
    }

    // When one other prime is the power of 7
    for (_, p) in &primes {
        let o = p.pow(7);

        if o > n { break }
        t += 1
    }


    t
}


#[test]
fn test_ranges() {
    assert_eq!(problem_501(100), 10);
    assert_eq!(problem_501(1000), 180);
    assert_eq!(problem_501(10_u64.pow(6)), 224427);
    //assert_eq!(problem_501(10_u64.pow(12)), 197_913_312_715)
}