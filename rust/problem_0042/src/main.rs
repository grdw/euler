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

fn is_triangle(n: &str) -> bool {
    let word_value = alphabet_value(n) as f64;
    let square = ((8.0 * word_value + 1.0).sqrt() - 1.0) / 2.0;

    square.fract() == 0.0
}

#[test]
fn test_is_triangle() {
    assert_eq!(is_triangle("SKY"), true);
    assert_eq!(is_triangle("ZZZZZZZZZZZZZZ"), false);
}

fn problem_42() -> usize {
    let contents = fs::read_to_string("p042_words.txt")
                      .unwrap_or("".to_string());

    contents.split(",").filter(|t| is_triangle(t)).count()
}

#[test]
fn test_problem_42() {
    assert_eq!(problem_42(), 162);
}
