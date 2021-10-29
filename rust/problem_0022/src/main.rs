fn main() {
    println!("Hello, world!");
}

use std::fs;

fn alphabet_value(name: &str) -> u32 {
    name
        .chars()
        .map(|c| {
            match (b'A'..=b'Z').position(|l| l as char == c) {
                Some(n) => (n + 1) as u32,
                None => 0
            }
        })
        .sum()
}

fn problem_22() -> u32 {
    let contents = fs::read_to_string("p022_names.txt")
                      .unwrap_or("".to_string());

    let mut names: Vec<&str> = contents.split(",").collect();

    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| alphabet_value(name) * (i as u32 + 1))
        .sum()
}

#[test]
fn test_problem_22() {
    assert_eq!(problem_22(), 871198282);
}
