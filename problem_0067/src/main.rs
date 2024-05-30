use std::fs;
use std::cmp;

fn main() {
    let t = fs::read_to_string("0067_triangle.txt").unwrap();

    let mut v: Vec<Vec<i128>> =
        t.split_terminator("\n").map(|n|{
            n.split(" ").map(|q| q.parse::<i128>().unwrap()).collect()
        }).collect();

    while v.len() > 1 {
        if let Some(row) = v.pop() {
            let index = v.len() - 1;
            let l = v[index].len();
            for j in 0..l {
                let m = row.get(j).unwrap_or(&0) + v[index][j];
                let n = row.get(j + 1).unwrap_or(&0) + v[index][j];
                v[index][j] = cmp::max(m, n);
            }
        }
    }
    println!("The answer is: {}", v[0][0]);
}
