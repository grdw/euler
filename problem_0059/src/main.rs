use std::fs;

fn main() {
    println!("Hello, world!");
}

const ASCII_MIN:u8 = 97;
const ASCII_MAX:u8 = 122;

#[derive(Debug, PartialEq)]
struct Password(u8, u8, u8);

#[derive(Debug)]
enum Action {
    AddTo2,
    AddTo1,
    AddTo0,
    Max
}

impl Iterator for Password {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let action: Action =
            if self.2 < ASCII_MAX {
                Action::AddTo2
            } else if self.1 < ASCII_MAX {
                Action::AddTo1
            } else if self.0 < ASCII_MAX {
                Action::AddTo0
            } else {
                Action::Max
            };

        match action {
            Action::AddTo2 => {
                self.2 += 1;
                Some(Password(self.0, self.1, self.2))
            }
            Action::AddTo1 => {
                self.1 += 1;
                self.2 = ASCII_MIN;
                Some(Password(self.0, self.1, self.2))
            },
            Action::AddTo0 => {
                self.0 += 1;
                self.1 = ASCII_MIN;
                self.2 = ASCII_MIN;
                Some(Password(self.0, self.1, self.2))
            },
            Action::Max => None
        }
    }
}

#[test]
fn test_password_iterator() {
    let mut password = Password(ASCII_MIN, ASCII_MIN, ASCII_MIN);
    let next = password.next().unwrap();

    assert_eq!(next, Password(97, 97, 98));
}

#[test]
fn test_password_iterator_max() {
    let mut password = Password(ASCII_MIN, ASCII_MIN, ASCII_MIN);
    let mut next = password.next().unwrap();
    for _ in 0..25 {
        next = password.next().unwrap();
    }

    assert_eq!(next, Password(97, 98, 97));
}

#[test]
fn test_password_iterator_three_cycles() {
    let mut password = Password(ASCII_MIN, ASCII_MIN, ASCII_MIN);
    let mut next = password.next().unwrap();
    for _ in 0..77 {
        next = password.next().unwrap();
    }

    assert_eq!(next, Password(97, 100, 97));
}

fn parse_password(text: &Vec<u8>, password: Password) -> String {
    let mut result = String::from("");
    let pw = vec![password.0, password.1, password.2];

    for i in 0..text.len() {
        let res = text[i] ^ pw[i % pw.len()];
        result.push(res as char);
    };
    result
}

#[test]
fn test_parse_password() {
    let list: Vec<u8> = vec![65, 65, 65];
    let answer = parse_password(&list, Password(42, 42, 42));

    assert_eq!(answer, "kkk");

    let list: Vec<u8> = vec![36,22,80,0,0,4,23,25,19,17,88,4,4,19,21];
    let answer = parse_password(&list, Password(100, 102, 99));

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
    let mut password = Password(ASCII_MIN, ASCII_MIN, ASCII_MIN - 1);

    'outer: loop {
        match password.next() {
            Some(next_password) => {
                let string = parse_password(&list, next_password);
                if string.contains(" and ") {
                    sum = string.as_bytes().iter().map(|x| *x as u64).sum();
                    break;
                }
            },
            None => break 'outer
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
