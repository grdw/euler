fn int_to_spoken_len(mut i: usize) -> usize {
    let mut spoken_lengths: Vec<usize> = vec![0; 1001];
    spoken_lengths[1] = "one".len();
    spoken_lengths[2] = "two".len();
    spoken_lengths[3] = "three".len();
    spoken_lengths[4] = "four".len();
    spoken_lengths[5] = "five".len();
    spoken_lengths[6] = "six".len();
    spoken_lengths[7] = "seven".len();
    spoken_lengths[8] = "eight".len();
    spoken_lengths[9] = "nine".len();
    spoken_lengths[10] = "ten".len();
    spoken_lengths[11] = "eleven".len();
    spoken_lengths[12] = "twelve".len();
    spoken_lengths[13] = "thirteen".len();
    spoken_lengths[14] = "fourteen".len();
    spoken_lengths[15] = "fifteen".len();
    spoken_lengths[16] = "sixteen".len();
    spoken_lengths[17] = "seventeen".len();
    spoken_lengths[18] = "eighteen".len();
    spoken_lengths[19] = "nineteen".len();
    spoken_lengths[20] = "twenty".len();
    spoken_lengths[30] = "thirty".len();
    spoken_lengths[40] = "forty".len();
    spoken_lengths[50] = "fifty".len();
    spoken_lengths[60] = "sixty".len();
    spoken_lengths[70] = "seventy".len();
    spoken_lengths[80] = "eighty".len();
    spoken_lengths[90] = "ninety".len();
    spoken_lengths[100] = "hundred".len();
    spoken_lengths[1000] = "thousand".len();

    let mut total = 0;
    let mut base = 100;

    if i == 100 || i == 1000 {
        // Adding 'one' for 100 and 1000
        total += 3;
    }

    while i > 0 {
        if spoken_lengths[i] > 0 {
            total += spoken_lengths[i];
            break
        }
        else {
            let (base_div, base_mod) = (i / base, i % base);

            if base_div > 0 && base == 100 {
                total += spoken_lengths[base_div];
                total += spoken_lengths[base];

                if base_mod > 0 {
                    // To count "and"
                    total += 3
                }
            }

            if base_div > 0 && base == 10 {
                total += spoken_lengths[base_div * base];
            }

            i -= base * base_div;
            base /= 10;
        }
    }

    total
}

#[test]
fn test_int_to_spoken_len() {
    assert_eq!(int_to_spoken_len(1), 3);
    // twenty one
    assert_eq!(int_to_spoken_len(21), 9);
    // one hundred
    assert_eq!(int_to_spoken_len(100), 10);
    // one hundred and fifteen = 20, don't count spaces
    assert_eq!(int_to_spoken_len(115), 20);
    // nine hundred and ninety nine
    assert_eq!(int_to_spoken_len(999), 24);
}

fn problem_17() -> usize {
    (1..=1000).map(|n| int_to_spoken_len(n)).sum()
}

#[test]
fn test_problem_17() {
    assert_eq!(problem_17(), 21124)
}

