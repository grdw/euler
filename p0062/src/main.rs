use std::collections::HashMap;

fn main() {
    let answer = problem_62();
    println!("The answer to problem 62 is: {}", answer);
}

// The brute force solution.
// Simply go cube by cube, and keep state.
// The alternative is to generate each permutation of the cube on the fly
// and count if there are 5 of them that are cubed. The downside here is
// that you have to test each permutation if they are cubed.
//
// So if you have a number like 452431232 that takes 9! variations to test
// which are way too many. Then my dumb solution is slightly better.
//
fn problem_62() -> u128 {
    let mut list: HashMap<String, Vec<u128>> = HashMap::new();
    let mut n: u128 = 1;

    loop {
        let cube = n.pow(3);
        let mut r_chars: Vec<char> = cube
            .to_string()
            .chars()
            .collect();

        r_chars.sort_by(|a, b| a.cmp(b)); // Sort chars
        let key: String = r_chars.into_iter().collect();

        list
            .entry(key.clone())
            .and_modify(|c| c.push(cube))
            .or_insert(vec![cube]);

        if let Some(n) = list.get(&key) {
            if n.len() == 5 {
                return n[0]
            }
        }

        n += 1;
    }
}

#[test]
fn test_problem_62() {
    assert_eq!(problem_62(), 127035954683);
}
