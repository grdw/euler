fn main() {
    let answer = problem_62();
    println!("The answer to problem 62 is: {}", answer);
}

fn problem_62() -> u128 {
    let result: u128;
    let mut list: Vec<Vec<u128>> = vec![];
    let mut start: u128 = 1;
    let mut prev_leng: usize = 1;

    loop {
        let cube = start.pow(3);
        // Detect changes in the length of the cube and drop off
        // values from the group
        let mut r_chars: Vec<char> = cube
            .to_string()
            .chars()
            .collect();

        r_chars.sort_by(|a, b| a.cmp(b)); // Sort chars

        let mut permute_list = list
            .iter_mut()
            .filter(|v| {
                let mut l_chars: Vec<char> = v[0]
                    .to_string()
                    .chars()
                    .collect();

                l_chars.sort_by(|a, b| a.cmp(b)); // Sort strings

                l_chars == r_chars
            });

        match permute_list.next() {
            Some(n) => {
                n.push(cube);

                if n.len() == 5 {
                    result = *n.iter().min().unwrap();
                    break;
                }
            }
            None => list.push(vec![cube])
        }

        if prev_leng < r_chars.len() {
            list.clear();
        }

        prev_leng = r_chars.len();
        start += 1;
    }

    result
}

#[test]
fn test_problem_62() {
    assert_eq!(problem_62(), 127035954683);
}
