use std::collections::HashMap;
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

#[derive(Debug)]
struct Route {
    direction: char,
    value: u32,
    list: usize
}

impl Route {
    fn new(direction: char, value: u32, list: usize) -> Self {
        Route { direction: direction, value: value, list: list }
    }

    fn left(value: u32, list: usize) -> Self {
        Route::new('L', value, list)
    }

    fn right(value: u32, list: usize) -> Self {
        Route::new('R', value, list)
    }
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


    let mut routes: HashMap<&u32, Vec<Route>> = HashMap::new();

    for (i, sub_list) in list.iter().enumerate() {
        for current in sub_list {
            for j in 0..list.len() {
                if i == j { continue };

                for righties in list[j]
                    .iter()
                    .filter(|v| digit_match(&current, v)) {
                        routes
                            .entry(current)
                            .and_modify(|v| v.push(
                                Route::right(*righties, j)
                            ))
                            .or_insert(vec![
                                Route::right(*righties, j)
                            ]);
                    }

                for lefties in list[j]
                    .iter()
                    .filter(|v| digit_match(v, &current)) {
                        routes
                            .entry(current)
                            .and_modify(|v| v.push(
                                Route::left(*lefties, j)
                            ))
                            .or_insert(vec![
                                Route::left(*lefties, j)
                            ]);
                    }
            }
        }
    }

    // Drop of all routes which only all point to the left
    // or all point to the right. These are not circular.
    routes.retain(|_, value| {
        !(value.iter().all(|r| r.direction == 'L') ||
          value.iter().all(|r| r.direction == 'R'))
    });

    let mut group = vec![];
    for (key, value) in &routes {
        println!("\n----\n{:?}", key);
        group.push(key);

        for route in value {
            // Find any route to the right, keep on going to
            // the right. If there are a substantial amount of
            match routes.get(&route.value) {
                Some(sub_routes) => {
                    let f: Vec<&Route> = sub_routes
                        .iter()
                        .filter(|r| route.direction == r.direction && route.list != r.list)
                        .collect();

                    println!("{:?}", f);
                },
                None => println!("Rien")
            }
        }
    }

    //loop {
    //    thread::sleep(Duration::from_millis(2000));
    //    windex += 1;

    //    let all_match = (0..group.len()).all(|i| {
    //        let x = group[i];
    //        let y = group[(i + 1) % group.len()];

    //        digit_match(&x, &y)
    //    });

    //    if all_match && group.len() == 6 {
    //        break;
    //    }
    //}
    0
}

#[test]
fn test_problem_61() {
    assert_eq!(problem_61(), 5);
}
