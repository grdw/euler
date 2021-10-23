fn multiples_of_3_and_5(n: i32) -> i32 {
    let mut total: i32 = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0  {
            total += i
        }
    };
    total
}

#[test]
fn find_multiples_of_3_and_5() {
    assert_eq!(multiples_of_3_and_5(10), 23);
    assert_eq!(multiples_of_3_and_5(1000), 233168);
}
