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

fn triangle_number_generator() -> Vec<u32> {
    let max = 364;
    let mut start = 1;
    let mut result = vec![];
    loop {
        let triangle = (
            (0.5 * start as f32) * (start as f32 + 1.0)
        ) as u32;

        result.push(triangle);
        start += 1;

        if triangle > max {
            break result
        }
    }
}

#[test]
fn test_triangle_number_generator() {
    let list = triangle_number_generator();
    assert_eq!(list.contains(&1), true);
    assert_eq!(list.contains(&2), false);
}

fn is_triangle(n: &str, list: &Vec<u32>) -> bool {
    let word_value = alphabet_value(n);

    list.contains(&word_value)
}

#[test]
fn test_is_triangle() {
    let list = triangle_number_generator();
    assert_eq!(is_triangle("SKY", &list), true);
    assert_eq!(is_triangle("ZZZZZZZZZZZZZZ", &list), false);
}

fn problem_42() -> usize {
    let contents = fs::read_to_string("p042_words.txt")
                      .unwrap_or("".to_string());

    let list = triangle_number_generator();

    contents.split(",").filter(|t| is_triangle(t, &list)).count()
}

#[test]
fn test_problem_42() {
    assert_eq!(problem_42(), 162);
}
