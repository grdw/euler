fn main() {
    println!("n gon ring of 3: {:?}", n_gon_ring(3));
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

fn n_gon_ring(x: u8) -> u64 {
    let mut m = 0;
	let mut t: Vec<u8> = (1..=x*2).collect();
	let mut v: Vec<u8> = t.clone();
    // This is just to test that all permutations have been checked
    t.reverse();

    while v != t {
        let mut v1 = vec![];
        for i in 0..x {
            let fi = i as usize;
            let fii = (i + x) as usize;
            let fiii = (((i + 1) % x) + x) as usize;

            v1.push(
                vec![
                    v[fi],
                    v[fii],
                    v[fiii]
                ]
            );
        }

        let comp = v1[0].iter().sum();
        if v1.iter().all(|x| x.iter().sum::<u8>() == comp) {
            let mut ts = String::new();

            let min_v = v1.iter().map(|n| n[0]).min().unwrap();
            while v1[0][0] != min_v {
                let x = v1.pop().unwrap();
                v1.insert(0, x);
            }

            for n in v1.into_iter().flatten() {
                let s = format!("{}", n);
                ts.push_str(&s);
            }

            let b = ts.parse::<u64>().unwrap();

            if b > m {
                m = b;
            }
        }
        next_perm(&mut v);
    }

    return m
}

#[test]
fn test_n_gon_ring() {
    assert_eq!(n_gon_ring(3), 432621513)
}
