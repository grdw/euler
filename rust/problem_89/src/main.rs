use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world!");
}

fn roman_to_int64(roman: &'static str) -> i64 {
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
fn all_roman_numerals() {
    let file = File::open("src/p089_roman.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{:?}", line.unwrap());
    }
}

#[test]
fn a_roman_numeral() {
    let roman_int64_1 = roman_to_int64("I");
    let roman_int64_4 = roman_to_int64("IIII");
    let roman_int64_4_2 = roman_to_int64("IV");
    let roman_int64_6 = roman_to_int64("VI");
    let roman_int64_19 = roman_to_int64("XIX");
    let roman_int64_49 = roman_to_int64("XLIX");

    assert_eq!(roman_int64_1, 1);
    assert_eq!(roman_int64_4, 4);
    assert_eq!(roman_int64_4_2, 4);
    assert_eq!(roman_int64_6, 6);
    assert_eq!(roman_int64_19, 19);
    assert_eq!(roman_int64_49, 49);
}
