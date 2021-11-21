fn sieve_of_erato(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n + 1];
    let max = (n as f64).sqrt() as usize;

    for i in 2..=max {
        if primes[i] {
            let mut j = i.pow(2);
            while j <= n {
                primes[j] = false;
                j += i
            }
        }
    }

    primes
}

fn unique_digits(number: usize) -> Vec<u8> {
    let mut digits = vec![];
    let mut digit_length = (number as f64).log10().ceil() as u32;

    while digit_length > 0 {
        let tens = 10_u64.pow(digit_length) as usize;
        let base = number % tens;
        let digit = (base / (tens / 10)) as u8;

        if !digits.contains(&digit) {
            digits.push(digit);
        }

        digit_length -= 1
    }

    digits
}

#[test]
fn test_unique_digits() {
    assert_eq!(unique_digits(6451), vec![6,4,5,1]);
    assert_eq!(unique_digits(121313), vec![1,2,3]);
}

use std::str;

fn replace_digit(slice: &str, digit: u8, replacement: u8) -> usize {
    let utf8_offset = 48;
    let digit_byte = &[digit + utf8_offset];
    let digit_str = str::from_utf8(digit_byte).unwrap();
    let replacement_byte = &[replacement + utf8_offset];
    let replacement_str = str::from_utf8(replacement_byte).unwrap();

    slice
        .replace(digit_str, replacement_str)
        .parse()
        .unwrap()
}

fn max_family_count(i: usize, sieve: &Vec<bool>) -> u64 {
    let string = i.to_string();
    let digits = unique_digits(i);

    digits
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            let start = if i == 0 { 1 } else { 0 };

            (start..=9)
                .map(|i| replace_digit(&string, *digit, i))
                .filter(|&nd| sieve[nd])
                .count()
        })
        .max()
        .unwrap_or(0) as u64
}

#[test]
fn test_max_family_count() {
    let sieve = sieve_of_erato(1_000_000);

    assert_eq!(max_family_count(13, &sieve), 6);
    assert_eq!(max_family_count(23, &sieve), 6);
    assert_eq!(max_family_count(6451, &sieve), 4);
    assert_eq!(max_family_count(111857, &sieve), 7);
    assert_eq!(max_family_count(121313, &sieve), 8);
}

fn problem_51() -> usize {
    let sieve = sieve_of_erato(1_000_000);
    let mut solution = 0;

    for n in 0..sieve.len() {
        if sieve[n] {
            let count = max_family_count(n, &sieve);
            if count == 8 {
                solution = n;
                break;
            }
        }
    }

    solution
}

#[test]
fn test_problem_51() {
    assert_eq!(problem_51(), 121313);
}
