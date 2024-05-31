use std::{thread, time::Duration};
fn main() {
    println!("Hello, world! {:?}", n_gon_ring(3, 1, 6));
}

fn next_perm(res: &mut Vec<u8>) {
    // Assume res = [0,2,1]
    let mut i = res.len() - 1; // this becomes 2
    while res[i - 1] >= res[i] { // is 2 >= 1 YES!
        i -= 1; // this becomes 1
    }

    let mut j = res.len(); // this becomes 3
    while res[j - 1] <= res[i - 1] { // is 1 <= 0 NO
        j -= 1;
    }

    res.swap(i - 1, j - 1); // SWAP 0 WITH 2 => [1,2,0]

    i += 1; // i becomes 2
    j = res.len(); // j becomes 3

    while i < j { // is 2 < 3, YEP
        res.swap(i - 1, j - 1); // SWAP 1 WITH 2 , STOP => [1,0,2]
        i += 1;
        j -= 1;
    }
}

fn n_gon_ring(x: usize, min: u8, max: u8) -> u64 {
    let mut m = 0;
	let mut t: Vec<u8> = (min..=max).collect();
	let mut v: Vec<u8> = t.clone();
    // This is just to test that all permutations have been checked
    t.reverse();

    loop {
        let n1 = v[0] + v[1] + v[3];
        let n2 = v[2] + v[3] + v[4];
        let n3 = v[1] + v[2] + v[5];

        if n1 == n2 && n2 == n3 {
            let mut b = 0;
            let mut v1 = vec![
                vec![v[0], v[1], v[3]],
                vec![v[4], v[3], v[2]],
                vec![v[5], v[2], v[1]]
            ];

            let min_v = v1.iter().map(|n| n[0]).min().unwrap();
            while v1[0][0] != min_v {
                let x = v1.pop().unwrap();
                v1.insert(0, x);
            }

            for (i, n) in v1.into_iter().flatten().enumerate() {
                b += 10_u64.pow((8 - i) as u32) * (n as u64);
            }

            if b > m {
                m = b;
            }
        }
        next_perm(&mut v);
        if v == t {
            break;
        }
    }

    return m
}

#[test]
fn test_n_gon_ring() {
    assert_eq!(n_gon_ring(3, 1, 6), 432621513)
}
