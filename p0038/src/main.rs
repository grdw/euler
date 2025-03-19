fn multiply_concatenate(n: u64) -> String {
    let mut multiplier = 1;
    let mut total = String::from("");
    let pandigital = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    loop {
        total.push_str(&format!("{}", n * multiplier));

        let mut chars = total.chars().collect::<Vec<char>>();
        let duplicates = pandigital
            .iter()
            .any(|d1| {
                chars
                    .iter()
                    .filter(|d2| *d2 == d1)
                    .count() > 1
            });

        if duplicates {
            break String::from("")
        }

        chars.sort();

        if chars == pandigital {
            break total
        }

        multiplier += 1;
    }
}

#[test]
fn test_multiply_concatenate() {
    assert_eq!(multiply_concatenate(192), String::from("192384576"));
    assert_eq!(multiply_concatenate(1), String::from("123456789"));
    assert_eq!(multiply_concatenate(2), String::from(""))
}

fn problem_38() -> String {
    let mut result = String::from("");

    for n in 1..=99_999 {
        let t = multiply_concatenate(n);
        if !t.is_empty() {
            result = t;
        }
    }
    result
}

#[test]
fn test_problem_38() {
    assert_eq!(problem_38(), String::from("932718654"));
}
