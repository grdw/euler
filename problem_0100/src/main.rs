fn main() {
    println!(
        "Answer: {:?}",
        first_arrangement_over(10_u128.pow(12))
    );
}

// If you don't know what Pell's equation is
// there's no way that you'll solve this.
// The basic idea I got right away that:
//
// D = discs
// B = blue discs
//
// (B / D) * ((B - 1) / (D - 1)) = (1 / 2)
// This can be simplified to:
// 2 * B * (B - 1) = D * (D - 1)
// which means
// 2B^2 - 2B = D^2 - D;
fn first_arrangement_over(n: u128) -> u128 {
    let mut prev_b = 15;
    let mut prev_d = 21;

    loop {
        let nb = (3 * prev_b) + (2 * prev_d) - 2;
        let nd = (4 * prev_b) + (3 * prev_d) - 3;
        prev_b = nb;
        prev_d = nd;

        if prev_d > n {
            return prev_b;
        }
    }
}

#[test]
fn test_first_arrangement_over() {
    assert_eq!(first_arrangement_over(15), 85);
    assert_eq!(first_arrangement_over(120), 493);
}
