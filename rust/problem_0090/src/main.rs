use itertools::Itertools;
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
//
// You need at least in both groups:
// 0, 1, 2, 3, 4, 5, 6, 8 or
// 0, 1, 2, 3, 4, 5, 8, 9
// else you can't make these numbers.
//

// This one is wrong but whatever ...
fn problem_90() -> i64 {
    let mut total_pos = 0;
    // This can probably be done a little smarter....
    let i: Vec<i8> = (0..10).collect();
    let j: Vec<i8> = (0..10).collect();
    let mut d1: Vec<Vec<i8>> = i.into_iter().combinations(6).collect();
    let mut d2: Vec<Vec<i8>> = j.into_iter().combinations(6).collect();
    d1.append(&mut d2);

    let d3: Vec<Vec<Vec<i8>>> = d1.into_iter().combinations(2).collect();

    let valid_group_1 = vec![0, 1, 2, 3, 4, 5, 6, 8];
    let valid_group_2 = vec![0, 1, 2, 3, 4, 5, 8, 9];

    for mut a in d3 {
        let mut temp = vec![];
        temp.append(&mut a[0]);
        temp.append(&mut a[1]);

        if temp.iter().all(|x| valid_group_1.contains(x)) {
            total_pos += 1
        } else if temp.iter().all(|x| valid_group_2.contains(x)) {
            total_pos += 1
        }
    }

    total_pos
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_problem_90() {
    assert_eq!(problem_90(), 1217);
}
