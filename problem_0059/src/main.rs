use std::fs;

fn main() {
    println!("The solution to problem_59 = {}", problem_59());
}

const ASCII_MIN:u8 = 97;
const ASCII_MAX:u8 = 122;

#[derive(Debug, PartialEq)]
struct Password(Vec<u8>);

impl Iterator for Password {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.iter().all(|&n| n == ASCII_MAX) {
            return None
        }

        let max = self.0.len();

        for i in (0..max).rev() {
            if self.0[i] < ASCII_MAX {
                self.0[i] += 1;
                break;
            } else {
                self.0[i] = ASCII_MIN;
            }
        }

        Some(Password(self.0.clone()))
    }
}

#[test]
fn test_password_iterator_simple_case() {
    let mut password = Password(vec![ASCII_MIN, ASCII_MIN, ASCII_MIN]);
    let next = password.next().unwrap();

    assert_eq!(next, Password(vec![97, 97, 98]));
}

#[test]
fn test_password_iterator_simple_over_max() {
    let mut password = Password(vec![97, 97, 122]);
    let next = password.next().unwrap();

    assert_eq!(next, Password(vec![97, 98, 97]));
}

#[test]
fn test_password_iterator_simple_over_max_0() {
    let mut password = Password(vec![97, 122, 122]);
    let next = password.next().unwrap();

    assert_eq!(next, Password(vec![98, 97, 97]));
}

#[test]
fn test_password_iterator_end() {
    let mut password = Password(vec![ASCII_MAX, ASCII_MAX, ASCII_MAX]);
    let next = password.next();

    assert_eq!(next, None);

    let mut password = Password(vec![97, 97, 97]);
    let mut next = password.next();

    for _ in 0..50_024 {
        next = password.next()
    }

    assert_eq!(next, None);
}

fn parse_password(text: &Vec<u8>, password: Password) -> String {
    let mut result = String::from("");
    let pw = password.0;

    for i in 0..text.len() {
        let res = text[i] ^ pw[i % pw.len()];
        result.push(res as char);
    };
    result
}

#[test]
fn test_parse_password() {
    let list: Vec<u8> = vec![65, 65, 65];
    let answer = parse_password(&list, Password(vec![42, 42, 42]));

    assert_eq!(answer, "kkk");

    let list: Vec<u8> = vec![36,22,80,0,0,4,23,25,19,17,88,4,4,19,21];
    let answer = parse_password(&list, Password(vec![100, 102, 99]));

    assert_eq!(answer, "@p3dfgs\u{7f}pu>g`uv");
}

fn problem_59() -> u64 {
    let contents = fs::read_to_string("p059_cipher.txt")
        .expect("Reads file succesfully");

    let list: Vec<u8> = contents
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let mut sum = 0;
    let mut password = Password(vec![ASCII_MIN, ASCII_MIN, ASCII_MIN - 1]);

    loop {
        match password.next() {
            Some(next_password) => {
                let string = parse_password(&list, next_password);
                if string.contains(" and ") {
                    sum = string.as_bytes().iter().map(|x| *x as u64).sum();
                    break;
                }
            },
            None => break
        }
    }

    return sum
}

#[test]
fn test_problem_59() {
    let n = problem_59();
    assert_eq!(n, 129448)
}

#[test]
fn test_int_to_ascii() {
    assert_eq!(65u8 as char, 'A')
}
