fn main() {
    println!("Answer: {}", detract_all(10_u128.pow(17)));
}

fn detract_all(n: u128) -> usize {
    return detract(n)
}

#[test]
fn test_detract_all() {
    assert_eq!(detract_all(100), 512);
}

fn detract(_n: u128) -> usize {
    return 0
}

#[test]
fn test_detract() {
    assert_eq!(detract(100), 4);
}
