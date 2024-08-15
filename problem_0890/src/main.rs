fn main() {
    println!("Hello, world!");
}

fn bin_par(n: u128) -> u128 {
    let mut i: usize = 2;
    let mut start = 2;
    while start * 2 < n {
        i += 1;
        start *= 2
    }
    println!("{} {}", start, i);

    return 0
}

#[test]
fn test_bin_par() {
    //assert_eq!(bin_par(7), 6);
    //assert_eq!(bin_par(7_u128.pow(7)), 6);
    assert_eq!(bin_par(7_u128.pow(777)), 6)
}
