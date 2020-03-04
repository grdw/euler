fn fibonacci_even(max: i32) -> i32 {
    let mut far = vec![1, 2];
    let mut total = far[1];

    loop {
        let n = far.remove(0);
        let m = far[0] + n;
        far.push(m);

        if m % 2 == 0 {
            total += m
        }

        if m >= max {
            break;
        }
    }
    total
}

#[test]
fn even_fibonnaci_numbers_test() {
    assert_eq!(fibonacci_even(8), 10);
    assert_eq!(fibonacci_even(55), 44);
    assert_eq!(fibonacci_even(4000000), 4613732);
}
