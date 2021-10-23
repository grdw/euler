// https://projecteuler.net/problem=109
// This is a problem around darts. So we probably need the dart board
// first in some data representation.
//
// Step 1:
// 1. Initiate a dart board.
// 2. There needs to be a distinction on S, D, T.
// 3. A possible representation is tuples: (1, 1), (2, 1), (3, 1)
//    where the first value is S,D,T and the second value the score
//    on the board.
//
// A player has 3 darts and we are excluding misses in this setup.
// How many double-outs are there for a score under a 100?
//
// So we know that for the total dart board it's 42336 with a max of 170.
//
// We know the amount of ways to go out with 6 = 11.
// For 1 it's 0 because there are no ways to go out.
// For 2 there's 1 [D1]
// For 3 there's 1 [D1, S1]

const MAX_OUT: i16 = 99;

fn generate_dartboard_values() -> Vec<(i16, i16)> {
    let mut dart_values: Vec<(i16, i16)> = vec![];
    for w in 1..=3 {
        for s in 1..=20 {
            dart_values.push((w, s));
        }
    }
    dart_values.push((1, 25)); // outer bull
    dart_values.push((2, 25)); // inner bull
    dart_values
}

fn num_outs(score: i16) -> i32 {
    // Basic counter:
    let mut n = 0;

    let dart_values = generate_dartboard_values();

    // This solution goes in reverse
    // So I throw the double first, then the second dart, then the third
    // dart.
    //
    // Throw 1:
    for (w1, s1) in &dart_values {
        let l = score - w1 * s1;

        // If the score drops below 0 or it's not a double, skip the cycle
        if l < 0 || *w1 != 2 { continue };

        // If it's finished after 1 throw, increase the counter and skip
        if l == 0 {
            n += 1;
            continue;
        }

        // This is some black magic:
        let mut states = vec![];

        // Throw 2
        for (w2, s2) in &dart_values {
            let l2 = l - w2 * s2;

            if l2 < 0 { continue };

            if l2 == 0 {
                n += 1;
                continue;
            }

            states.push((w2, s2));

            // Throw 3
            for (w3, s3) in &dart_values {
                let l3 = l2 - w3 * s3;

                if l3 < 0 { continue };

                // I HAVE NO IDEA WHY THIS WORKS AND I DON'T
                // TRUST IT
                if l3 == 0 && states.iter().any(|(wc, sc)| *wc == w3 && *sc == s3) {
                    n += 1;
                    continue;
                }
            }
        }
    };

    n
}

fn problem_109() -> i32 {
    let mut n = 0;

    for out in 2..=MAX_OUT {
        n += num_outs(out);
    }

    n
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_problem_109() {
    assert_eq!(problem_109(), 38182)
}

#[test]
fn test_outs_for_1() {
    assert_eq!(num_outs(1), 0); // []
}

#[test]
fn test_outs_for_2() {
    assert_eq!(num_outs(2), 1); // [D1]
}

#[test]
fn test_outs_for_3() {
    assert_eq!(num_outs(3), 1); // [D1, S2]
}

#[test]
fn test_outs_for_6() {
    assert_eq!(num_outs(6), 11);
}

#[test]
fn test_outs_for_169() {
    assert_eq!(num_outs(169), 0);
}

#[test]
fn test_outs_for_170() {
    assert_eq!(num_outs(170), 1);
}
