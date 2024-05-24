use std::collections::HashSet;

fn main() {
    println!("Hello, world! {:?}", graph(45));
}

fn is_prime(number: i16) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as i16;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn divisors(i: i16) -> Vec<i16> {
    let mut list = vec![];
    for n in 2..i {
        if i % n == 0 && is_prime(i / n) {
            list.push(n)
        }
    }
    return list
}

fn graph(n: i16) -> i16 {
    let mut levels: Vec<HashSet<i16>> = vec![];
    let mut level = 0;
    let mut list = vec![n];
    loop {
        let m = list.pop();
        match m {
            Some(n) => {
                let nodes = divisors(n);

            },
            None => break
        }
    }
    0
}

#[test]
fn test_graph() {
    assert_eq!(graph(45), 2);
    assert_eq!(graph(5040), 12);
}
