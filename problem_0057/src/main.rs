fn main() {
    println!("Hello, world!");
}

fn expand_sqrt_of_2(cycles: i32) -> (i32, f32) {
    let mut start_n = 1;
    let mut start_d = 2;
    for _ in 0..cycles {
        start_d += 0.5;
    };

    (0, 0.0)
}

#[test]
fn test_expand_sqrt_of_2() {
    assert_eq!(expand_sqrt_of_2(2), (1, 2.0));
    //assert_eq!(expand_sqrt_of_2(1), 1.5);
    //assert_eq!(expand_sqrt_of_2(2), 1.4);
    //assert_eq!(expand_sqrt_of_2(3), 1.41666);
    //assert_eq!(expand_sqrt_of_2(4), 1.41379);
}
