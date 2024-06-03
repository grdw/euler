use itertools::Itertools;
use std::collections::HashSet;
// https://projecteuler.net/problem=90
// There are 2 groups of numbers with each len = 6
// And both of these 2 groups should be able to make:
// 01, 04, 09, 16, 25, 36, 49, 64, 81
//
// 8 1
// ^
//  ---- The first number can be from the second group
//
// Problem in steps:
// 1. Make every possible len = 6 sub group of (0-9) where each N is unique.
// 2. Repeat step 1 for the second dice.
// 3. Example: {0, 5, 6, 7, 8, 9} and {1, 2, 3, 4, 8, 9}
//
// What are all possible values you can make?
// {1, 2, 3, 4, 8, 9,
// 51, 52, 53, 54, 58, 59,
// 61, 62, 63, 64, 68, 69,
// 71, 72, 73, 74, 78, 79,
// 81, 82, 83, 84, 88, 89,
// 91, 92, 93, 94, 98, 99,
// 10, 15, 16, 17, 18, 19,
// 20, 25, 26, 27, 28, 29,
// 30, 35, 36, 37, 38, 39,
// 40, 45, 46, 47, 48, 49,
// 80, 85, 86, 87, 88, 89,
// 90, 95, 96, 97, 98, 99}
//
// Does this group contain all the square roots?
// Yes
//
// You have 9 numbers (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
// See it as having 9 balls to pick from 2 buckets (like Lingo):
// So:
// (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
// (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
//
// And from each of these 2 groups (buckets) you pick 6 random numbers,
// however:
// 1. The 6 counts as a 6 and a 9.
// 2. The 9 counts as a 6 and a 9.
// 3. To make all these numbers: 1, 4, 9, 16, 25, 36, 49, 64, 81

const SQUARES: [i8; 9] = [
    1,
    4,
    9,
    16,
    25,
    36,
    49,
    64,
    81
];

fn problem_90() -> usize {
    let i: Vec<i8> = (0..10).collect();
    let d1: Vec<Vec<i8>> = i.into_iter().combinations(6).collect();
    let mut s: HashSet<Vec<&Vec<i8>>> = HashSet::new();

    for g1 in &d1 {
        for g2 in &d1 {
            if can_make_all_squares(g1, g2) {
                let mut groups = vec![g1, g2];
                groups.sort();
                s.insert(groups);
            }
        }
    }

    s.len()
}

fn can_make_all_squares(g1: &Vec<i8>, g2: &Vec<i8>) -> bool {
    let mut combinations = vec![];
    for x in g1 {
        for y in g2 {
            if x == &6 {
                combinations.push(90 + y);
                combinations.push(9 + y * 10);
            }
            if y == &6 {
                combinations.push(90 + x);
                combinations.push(9 + x * 10);
            }
            if x == &9 {
                combinations.push(60 + y);
                combinations.push(6 + y * 10);
            }
            if y == &9 {
                combinations.push(60 + x);
                combinations.push(6 + x * 10);
            }
            combinations.push(x * 10 + y);
            combinations.push(x + y * 10);
        }
    }

    return SQUARES.iter().all(|sq| combinations.contains(&sq))
}

fn main() {
    println!("The answer is {}", problem_90());
}

#[test]
fn test_problem_90() {
    assert_eq!(problem_90(), 1217);
}

#[test]
fn test_can_make_all_squares() {
    assert_eq!(
        can_make_all_squares(
            &vec![1, 2, 3, 4, 8, 9],
            &vec![0, 5, 6, 7, 8, 9]
        ),
        true
    );

    assert_eq!(
        can_make_all_squares(
            &vec![1, 2, 3, 4, 8, 6],
            &vec![0, 5, 6, 7, 8, 9]
        ),
        true
    );

    assert_eq!(
        can_make_all_squares(
            &vec![0, 5, 6, 7, 8, 9],
            &vec![1, 2, 3, 4, 8, 6]
        ),
        true
    );

    assert_eq!(
        can_make_all_squares(
            &vec![0, 5, 1, 7, 8, 9],
            &vec![1, 2, 3, 4, 8, 9]
        ),
        true
    );
}
