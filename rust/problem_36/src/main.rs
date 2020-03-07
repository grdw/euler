fn is_palindrome(string: String) -> bool {
    let len = string.len() - 1;

    if len == 0 {
        return true
    }

    let end = (len + 1) / 2;
    let r: Vec<usize> = (0..end).collect();

    r.iter().all(|x| {
        let l = string.chars().nth(*x);
        let r = string.chars().nth(len - *x);
        l == r
    })
}

fn sum_palindrome_base2_10(digits: u32) -> u32 {
    let mut sum_pal = 0;
    for i in 0..digits {
        let i_base10 = format!("{}", i);
        let i_base2 = format!("{:b}", i);

        if is_palindrome(i_base10) && is_palindrome(i_base2) {
            sum_pal += i;
        }
    }
    sum_pal
}

#[test]
fn palindrome_numbers_test() {
    assert_eq!(sum_palindrome_base2_10(586), 1055);
    assert_eq!(sum_palindrome_base2_10(1_000_000), 872187);
}
