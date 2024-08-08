fn main() {
    println!("Answer: {}", lowest_partition(1_000_000));
}

fn lowest_partition(divisor: u64) -> u64 {
    return 0
}

fn partitions(n: u64) -> u64 {
    for i in 1..10 {
        println!("{} {}", pentagonal(i), pentagonal(-i));
    }
    return 0
}

fn pentagonal(x: i64) -> i64 {
    return (x * (3 * x - 1)) / 2
}

#[test]
fn test_partitions() {
    assert_eq!(partitions(5), 7)
}
