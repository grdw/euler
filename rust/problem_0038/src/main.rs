fn multiply_concatenate(n: u64) -> Option<String> {
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
            break None
        }

        chars.sort();

        if chars == pandigital {
            break Some(total)
        }

        multiplier += 1;
    }
}

#[test]
fn test_multiply_concatenate() {
    assert_eq!(multiply_concatenate(192), Some(String::from("192384576")));
    assert_eq!(multiply_concatenate(1), Some(String::from("123456789")));
    assert_eq!(multiply_concatenate(2), None)
}

fn problem_38() -> String {
    let mut max = 1;
    let mut result = String::from("");

    for n in 1..=99_999 {
        match multiply_concatenate(n) {
            Some(t) => {
                if n > max {
                    result = t;
                    max = n;
                }
            },
            None => {}
        }
    }
    result
}

#[test]
fn test_problem_38() {
    assert_eq!(problem_38(), String::from("932718654"));
}
