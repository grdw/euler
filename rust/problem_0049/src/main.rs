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

const BOUND: u64 = 999;

fn gaps(vec: &Vec<u64>) -> Vec<u64> {
    let l = vec.len();
    let mut differences: Vec<(usize, usize, u64)> = vec![];
    let mut train = vec![];

    for v in 0..l {
        for w in 0..v {
            let value = vec[v] - vec[w];
            let diff = (v, w, value);

            for t in &differences {
                if t.0 == w && t.2 == value && value > BOUND {
                    train.push(vec[t.1]);
                    train.push(vec[w]);
                    train.push(vec[v]);
                }
            }

            differences.push(diff);
        }
    }

    train
}

#[test]
fn test_even_gaps() {
    assert_eq!(gaps(&vec![1, 2, 4, 7]), vec![]);
    assert_eq!(gaps(&vec![1, 2, 4, 9, 12]), vec![]);
    assert_eq!(
        gaps(

            &vec![
                1487, 1847, 4817, 4871, 7481, 7841, 8147, 8741
            ]
        ),
        vec![1487, 4817, 8147]
    );

    assert_eq!(
        gaps(
            &vec![
                1627, 2617, 2671, 6217, 6271, 7621
            ]
        ),
        vec![]
    )
}


fn four_digit_primes() -> Vec<u64> {
    (1000..=9999)
        .filter(|&n| is_prime(n))
        .collect()
}

fn sort(string: &String) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

fn strict_match(string: &String, other_string: &String) -> bool {
    sort(string) == sort(other_string)
}

#[test]
fn test_strict_match() {
    assert_eq!(strict_match(&String::from("1009"), &String::from("1019")), false);
    assert_eq!(strict_match(&String::from("1013"), &String::from("1031")), true);
}

fn problem_49() -> Vec<u64> {
    let mut groups: Vec<Vec<u64>> = vec![];
    let mut primes = four_digit_primes();
    let mut result = vec![];
    let l = primes.len();

    for i in 0..l {
        let pi = primes[i];
        let pi_string = pi.to_string();

        for j in i+1..l {
            let pj = primes[j];
            let pj_string = pj.to_string();

            if primes[j] > 0 && strict_match(&pi_string, &pj_string) {
                match groups.get_mut(i) {
                    Some(v) => { v.push(pj) },
                    None => {
                        let group = vec![pi, pj];
                        primes[j] = 0;
                        groups.push(group);
                    }
                }
            }
        }
    }

    for g in &groups {
        if g.len() > 2 {
            let snake = gaps(&g);
            if snake.len() > 0 && !g.contains(&1487) {
                result = snake;
                break;
            }
        }
    }

    result
}

#[test]
fn test_problem_49() {
    assert_eq!(problem_49(), vec![2969, 6299, 9629]);
}
