fn main() {
    println!("Hello, world!");
}

fn count_spiral_grid(size: u64) -> u64 {
    let mut start = 1;
    let mut total = 0;
    let mut increase = 2;
    let mut cycles = 0;

    while start <= size.pow(2) {
        if cycles > 0 && cycles % 4 == 0 {
            increase += 2;
        }

        total += start;
        start += increase;
        cycles += 1;
    }

    total
}

#[test]
fn test_make_spiral_grid() {
    assert_eq!(count_spiral_grid(3), 25);
    assert_eq!(count_spiral_grid(5), 101);
    assert_eq!(count_spiral_grid(7), 261);
    assert_eq!(count_spiral_grid(1001), 669171001);
}
