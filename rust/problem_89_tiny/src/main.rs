use std::fs;
use regex::Regex;

// https://projecteuler.net/problem=89
// Cheated from:
// https://www.mathblog.dk/project-euler-89-develop-a-method-to-express-roman-numerals-in-minimal-form/
fn problem_89() -> usize {
    let file = fs::read_to_string("src/p089_roman.txt").expect("boom");
    let re = Regex::new(r"DCCCC|LXXXX|VIIII|CCCC|XXXX|IIII").unwrap();
    let result = re.replace_all(&file, "kk");
    file.len() - result.len()
}

fn main() {
    println!("{}", problem_89());
}


#[test]
fn test_all_roman_numerals() {
    assert_eq!(problem_89(), 743);
}
