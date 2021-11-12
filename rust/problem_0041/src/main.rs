fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false
    }

    let mut is_prime: bool = true;
    let end = (number as f64).sqrt().floor() as u64;

    for i in 2..end+1 {
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}

fn vec_to_int(vec: &Vec<char>) -> u64 {
    vec
      .iter()
      .collect::<String>()
      .parse()
      .unwrap()
}

fn highest_heap_prime(mut vector: Vec<char>) -> u64 {
    let mut max: u64 = 0;
    let mut result: Vec<usize> = vec![0; vector.len()];
    let mut i = 0;

    while i < vector.len() {
        if result[i] < i {
            if i % 2 == 0 {
                vector.swap(0, i);
            } else {
                vector.swap(result[i], i);
            }

            let n: u64 = vec_to_int(&vector);
            if n > max && is_prime(n) {
                max = n;
            }
            result[i] += 1;
            i = 0;
        } else {
            result[i] = 0;
            i += 1
        }
    }

    max
}

fn problem_41() -> u64 {
    let mut vector = vec!['9', '8', '7', '6', '5', '4', '3', '2', '1'];
    let mut max = 0;

    for _ in 0..vector.len() {
        vector.remove(0);

        let hhp = highest_heap_prime(vector.clone());
        if hhp > max {
            max = hhp;
            break;
        }
    }

    max
}

#[test]
fn test_highest_pandigital_prime() {
    assert_eq!(problem_41(), 7652413)
}
