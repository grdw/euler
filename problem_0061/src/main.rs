use std::{thread, time::Duration};

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

fn digit_match(x: &u32, y: &u32) -> bool {
    let last_digits = x % 100;
    let first_digits = (y - (y % 100)) / 100;

    last_digits == first_digits
}

fn problem_61() -> u32 {
    let mut list = vec![
        create_list(&triangle),
        create_list(&square),
        create_list(&pentagonal),
        create_list(&hexagonal),
        create_list(&heptagonal),
        create_list(&octagonal)
    ];


    let mut group = vec![];
    let mut index = 0;
    let mut filter = vec![index];

    loop {
        let current = list[index][0];

        for i in 0..list.len() {
            if !filter.contains(&i) {
                let mut candidates: Vec<&u32> = list[i]
                    .iter()
                    .filter(|v| digit_match(&current, v))
                    .collect();

                if candidates.is_empty() {
                    continue;
                }

                println!("{}", current);
                println!("{:?} from {}", candidates, i);
            }
        }

        break;
    }
    0
}

#[test]
fn test_problem_61() {
    assert_eq!(problem_61(), 5);
}
