fn coin_values(value: usize) -> usize {
    let coins: Vec<usize> = vec![
        200,
        100,
        50,
        20,
        10,
        5,
        2,
        1
    ];

    let mut cylinder = vec![0; value + 1];
    cylinder[0] = 1;

    for c in &coins {
        for i in 0..=(value - c) {
            cylinder[i + *c] += cylinder[i];
        }
    }

    cylinder[value]
}

#[test]
fn test_problem_31() {
    assert_eq!(coin_values(200), 73682);
}
