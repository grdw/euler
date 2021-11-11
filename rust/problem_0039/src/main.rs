fn main() {
    println!("Hello, world!");
}

fn calculate_c(a: u64, b: u64) -> Option<u64> {
    let c = (a.pow(2) + b.pow(2)) as f64;
    let sqrt = c.sqrt();

    if sqrt.fract() == 0.0 {
        Some(sqrt as u64)
    } else {
        None
    }
}

#[test]
fn test_pythagoras() {
    assert_eq!(calculate_c(30, 40), Some(50));
    assert_eq!(calculate_c(499, 2), None);
    assert_eq!(calculate_c(1, 120), None);
}

fn problem_39() -> u64 {
    let max: u64 = 1000;
    let max_r: u64 = max - 2;
    let mut triplets = vec![0; (max + 1) as usize];
    let mut max_index = 0;
    let mut max_f = 0;

    for a in 1..=max_r {
        for b in a..=max_r {
            if let Some(c) = calculate_c(a, b) {
                let p = a + b + c;

                if p <= max {
                    if p == max {
                        println!("{}, {}, {}", a, b, c);
                    }
                    triplets[p as usize] += 1;
                }
            }
        }
    }

    for (i, f) in triplets.iter().enumerate() {
        if *f > max_f {
            max_f = *f;
            max_index = i;
        }
    }

    max_index as u64
}

#[test]
fn test_problem_39() {
    assert_eq!(problem_39(), 840);
}
