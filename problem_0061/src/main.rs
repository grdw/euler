fn main() {
    println!("Hello, world!");
}

const MIN: u32 = 1000;
const MAX: u32 = 9999;

fn is_useful(n: u32) -> bool {
    n >= MIN && n <= MAX && n % 100 > 9
}

fn create_list(func: &dyn Fn(u32) -> u32) -> Vec<u32> {
    let mut n = 1;
    let mut list = vec![];
    loop {
        let t = func(n);

        if t > MAX {
            break;
        }

        if is_useful(t) {
            list.push(t)
        }

        n += 1
    }

    list
}

fn triangle(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn square(n: u32) -> u32 {
    n * n
}

fn pentagonal(n: u32) -> u32 {
    (n * ((3 * n) - 1)) / 2
}

fn hexagonal(n: u32) -> u32 {
    n * ((2 * n) - 1)
}

fn heptagonal(n: u32) -> u32 {
    (n * ((5 * n) - 3)) / 2
}

fn octagonal(n: u32) -> u32 {
    n * ((3 * n) - 2)
}

#[test]
fn test_create_list() {
    let tri_list = create_list(&triangle);
    assert_eq!(tri_list.len(), 88);
    assert_eq!(tri_list[0], 1035);
    assert_eq!(tri_list[tri_list.len() - 1], 9870);

    let square_list = create_list(&square);
    assert_eq!(square_list.len(), 53);
    assert_eq!(square_list[0], 1024);
    assert_eq!(square_list[square_list.len() - 1], 9216);

    let square_list = create_list(&octagonal);
    assert_eq!(square_list.len(), 30);
    assert_eq!(square_list[0], 1045);
    assert_eq!(square_list[square_list.len() - 1], 9976);
}

fn digit_match(x: u32, y: &u32) -> bool {
    let last_digits = x % 100;
    let first_digits = (y - (y % 100)) / 100;

    last_digits == first_digits
}

use std::{thread, time::Duration};
fn problem_61() -> u32 {
    let digits = [
        create_list(&triangle),
        create_list(&square),
        create_list(&pentagonal),
        create_list(&hexagonal),
        create_list(&heptagonal),
        create_list(&octagonal)
    ];

    let mut group = vec![];
    let mut d_index = 0;
    let mut index = vec![0; digits.len()];

    loop {
        thread::sleep(Duration::from_millis(1000));
        let mut pick = digits[d_index][index[d_index]];
        let next_list = &digits[(d_index + 1) % digits.len()];

        let mut next_index = None;
        for i in 0..next_list.len() {
            if digit_match(pick, &next_list[i]) {
                println!("{} / {} -- {} {} {:?}", pick, next_list[i], d_index, group.len(), group);
                if d_index == 5 {
                    if digit_match(pick, &group[0]) {
                        group.push(pick);
                        next_index = Some(i);
                    }
                } else {
                    group.push(pick);
                    next_index = Some(i);
                }
            }
        }

        match next_index {
            Some(i) => {
                d_index += 1;
                index[d_index % digits.len()] = i;
            },
            None => {
                group.truncate(d_index);
                d_index = group.len();
                index[d_index] += 1;
            }
        }

        println!("{:?} d: {} i: {:?}", group, d_index, index);
        if group.len() == 6 {
            break;
        }
    }

    group.iter().sum()
}

#[test]
fn test_problem_61() {
    assert_eq!(problem_61(), 5);
}
