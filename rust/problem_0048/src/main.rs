fn power_to_10(i: u64) -> u64 {
    let mut tens: u64 = 10;
    let mut n = 1;
    let mut i_base = i;
    let mut total = 0;
    let max = 10;

    while n < i {
        while i_base > 0 {
            let base = i_base % tens;
            total += base * i;

            if tens > 10_u64.pow(max) {
                break;
            }

            tens *= 10;
            i_base -= base;
        }

        i_base = total;
        if i - n > 1 {
            total = 0;
        }

        tens = 10;
        n += 1;
    }

    total
}

#[test]
fn test_power_to_10() {
    assert_eq!(power_to_10(11), 285311670611); // last 11 digits
    assert_eq!(power_to_10(14), 206825558016); // last 14 digits
}

fn problem_48() -> u64 {
    let mut start: u64 = 0;
    let mut total = 1; // power_to_10(1) returns 0, so 1^1 is where it starts

    while start < 1000 {
        start += 1;
        total += power_to_10(start);
    }

    total % 10_u64.pow(10)
}

#[test]
fn test_problem_48() {
    assert_eq!(problem_48(), 9110846700);
}
