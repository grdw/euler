use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world!");
}

fn int64_to_roman(i: i64) -> String {
    // I cheated a little here and took the table from:
    // https://www.rapidtables.com/convert/number/how-number-to-roman-numerals.html
    let mut k = i;
    let mut chars = vec![];
    let nums = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1)
    ];

    for (c, n) in nums {
        let div = k / n;

        if div == 0 {
            continue
        }

        for _ in 0..div {
            chars.push(c);
            k -= n;
        }
    }

    chars.into_iter().collect()
}

fn roman_to_int64(roman: String) -> i64 {
    let mut prev = 0;

    roman.chars().rev().fold(0, |total, c| {
        let mut num = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid roman")
        };

        if prev > num {
            num *= -1;
        }

        prev = num;
        total + num
    })
}

#[test]
fn test_all_roman_numerals() {
    let file = File::open("src/p089_roman.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let roman = line.unwrap();
        println!("{:?}", roman_to_int64(roman));
    }
}

#[test]
fn test_roman_to_int64() {
    let roman_int64_1 = roman_to_int64(String::from("I"));
    let roman_int64_4 = roman_to_int64(String::from("IIII"));
    let roman_int64_4_2 = roman_to_int64(String::from("IV"));
    let roman_int64_6 = roman_to_int64(String::from("VI"));
    let roman_int64_19 = roman_to_int64(String::from("XIX"));
    let roman_int64_49 = roman_to_int64(String::from("XLIX"));

    assert_eq!(roman_int64_1, 1);
    assert_eq!(roman_int64_4, 4);
    assert_eq!(roman_int64_4_2, 4);
    assert_eq!(roman_int64_6, 6);
    assert_eq!(roman_int64_19, 19);
    assert_eq!(roman_int64_49, 49);
}

#[test]
fn test_int64_to_roman() {
    let int64_1 = 1;
    let int64_2601 = 2601;
    let int64_4 = 4;
    let int64_6 = 6;
    let int64_19 = 19;
    let int64_49 = 49;

    assert_eq!(int64_to_roman(int64_1), String::from("I"));
    assert_eq!(int64_to_roman(int64_2601), String::from("MMDCI"));
    assert_eq!(int64_to_roman(int64_4), String::from("IV"));
    assert_eq!(int64_to_roman(int64_6), String::from("VI"));
    assert_eq!(int64_to_roman(int64_19), String::from("XIX"));
    assert_eq!(int64_to_roman(int64_49), String::from("XLIX"));
}
