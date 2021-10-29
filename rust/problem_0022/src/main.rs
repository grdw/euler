fn main() {
    println!("Hello, world!");
}

use std::fs;

fn problem_22() -> u32 {
    let contents = fs::read_to_string("p022_names.txt")
                      .unwrap_or("".to_string());

    let mut names: Vec<&str> = contents.split(",").collect();
    let alphabet: Vec<char> = (b'A'..=b'Z').map(|n| n as char).collect();

    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let alphabet_value: u32 = name
                .chars()
                .map(|c| {
                    match alphabet.iter().position(|&l| l == c) {
                        Some(n) => (n + 1) as u32,
                        None => 0
                    }
                })
                .sum();

            alphabet_value * (i as u32 + 1)
        })
        .sum()
}

#[test]
fn test_problem_22() {
    assert_eq!(problem_22(), 871198282);
}
