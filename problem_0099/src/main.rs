use std::fs;
use std::cmp::Ordering;

fn main() {
    let f = fs::read_to_string("0099_base_exp.txt").unwrap();
    let mut values = vec![];
    for line in f.split_terminator("\n") {
        let (num_s, exp_s) = line.split_once(",").unwrap();
        let num = num_s.parse::<f64>().unwrap();
        let exp = exp_s.parse::<f64>().unwrap();
        values.push((num, exp));
    }
    println!("Answer: {}", highest_exp(&values));
}

fn highest_exp(list: &Vec<(f64, f64)>) -> usize {
    let mut sorted_list = list.clone();
    sorted_list.sort_by(|(a, ae), (b, be)| compare_exps(*a, *ae, *b, *be));
    return 1 + list
        .iter()
        .position(|&o| o == sorted_list[0])
        .unwrap()
}

fn compare_exps(a: f64, ae: f64, b: f64, be: f64) -> Ordering {
    if a.log10() * ae > b.log10() * be {
        return Ordering::Less
    } else {
        return Ordering::Greater
    }
}

#[test]
fn test_highest_exp() {
    assert_eq!(highest_exp(&vec![(2.0, 11.0), (3.0, 7.0)]), 2);
}
