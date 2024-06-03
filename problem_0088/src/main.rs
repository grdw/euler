use std::collections::HashSet;

fn main() {
    println!("Answer: {}", sum_group(2, 12000));
}

fn sum_group(min: u32, max: u32) -> u32 {
    let mut answer = 0;
    let mut s: HashSet<u32> = HashSet::new();

    for n in min..=max {
        let p = sum_l(n);
        s.insert(p);
    }

    for v in &s {
        answer += v;
    }

    return answer
}

fn sum_l(n: u32) -> u32 {
    let mut counter = 10_u32.pow(n - 1);
    let mut answer = u32::MAX;

    println!("GOING FOR: {}", n);

    loop {
        counter += 1;

        let num_length = ((counter as f64).log10() + 1.0).floor() as u32;
        if num_length == n + 1 {
            break;
        }

        let mut pr: u32 = 1;
        let mut sm: u32 = 0;
        for i in 0..num_length {
            let n = counter % (10_u32.pow(i + 1)) / 10_u32.pow(i);
            pr *= n;
            sm += n;
        }

        if pr == sm && pr < answer {
            answer = pr;
        }
    }

    return answer
}

#[test]
fn test_sum_group() {
    assert_eq!(sum_group(2, 2), 4);
    assert_eq!(sum_group(2, 6), 30);
    assert_eq!(sum_group(2, 12), 61);
}
