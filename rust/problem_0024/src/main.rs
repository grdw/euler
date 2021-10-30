fn main() {
    println!("Hello, world!");
}

fn factorial(mut i: u64) -> u64 {
    let mut total = 2;
    while i > 2 {
        total *= i;
        i -= 1
    }
    total
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(3), 6);
}

fn reverse(vec: &mut Vec<u8>, mut a: usize, mut b: usize) {
    while a < b {
        vec.swap(a, b);
        a += 1;
        b -= 1;
    }
}

fn ord_smith(vec: &mut Vec<u8>, s: usize, count: &mut u32, max: u32) {
    if s == vec.len() - 1 {
        *count += 1;
        if *count == max {
            println!("RESULT: {:?} {}", vec, count);
        }
        return
    }

    for i in 0..vec.len() - s {
        if i > 0 {
            reverse(vec, s + 1, vec.len() - 1);
            vec.swap(s, s + i);
        }
        ord_smith(vec, s + 1, count, max);
    }
}

#[test]
fn test_ord_smith() {
    let mut count = 0;
    let mut group = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    ord_smith(&mut group, 0, &mut count, 1000000);
}
