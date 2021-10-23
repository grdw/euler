fn is_palindrome(x: i32) -> bool {
    let string = x.to_string();
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

fn max_palindrome_digits(digits: u32) -> i32 {
    let mut highest_pal = 0;
    let mut max_number = 10_i32.pow(digits) - 1;

    'outer: while max_number > 0 {
        let mut inner_max = max_number;

        while inner_max > 0 {
            let factor = max_number * inner_max;

            if is_palindrome(factor) {
                highest_pal = factor;
                break 'outer;
            }
            inner_max -= 1
        }
        max_number -= 1
    }
    highest_pal
}

#[test]
fn is_palindrome_test() {
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(12), false);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(101), true);
    assert_eq!(is_palindrome(1001), true);
    assert_eq!(is_palindrome(10101), true);
}

#[test]
fn palindrome_numbers_test() {
    assert_eq!(max_palindrome_digits(2), 9009);
    assert_eq!(max_palindrome_digits(3), 90909);
    assert_eq!(max_palindrome_digits(4), 99000099);
}
