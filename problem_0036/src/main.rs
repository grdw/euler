fn is_palindrome(string: String) -> bool {
    let len = string.len() - 1;
    let end = string.len() / 2;

    string[0..end]
        .chars()
        .enumerate()
        .all(|(i,n)| n == string.chars().nth(len - i).unwrap())
}

fn sum_palindrome_base2_10(digits: u32) -> u32 {
    (0..digits).fold(0, |acc, i| {
        let i_base10 = format!("{}", i);
        let i_base2 = format!("{:b}", i);

        if is_palindrome(i_base10) && is_palindrome(i_base2) {
            acc + i
        } else {
            acc
        }
    })
}

#[test]
fn palindrome_numbers_test() {
    assert_eq!(sum_palindrome_base2_10(586), 1055);
    assert_eq!(sum_palindrome_base2_10(1_000_000), 872187);
}
