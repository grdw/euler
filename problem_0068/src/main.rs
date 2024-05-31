fn main() {
    println!("n gon ring of 3: {:?}", n_gon_ring(3, usize::MAX));
    println!("n gon ring of 5: {:?}", n_gon_ring(5, 16));
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

fn n_gon_ring(x: u8, limit: usize) -> u64 {
    let indices = get_indices(x as usize);

    let mut m = 0;
	let mut t: Vec<u8> = (1..=x*2).collect();
	let mut v: Vec<u8> = t.clone();
    // This is just to test that all permutations have been checked
    t.reverse();

    while v != t {
        let mut v1: Vec<Vec<u8>> = indices
            .iter()
            .map(|i| vec![v[i[0]], v[i[1]], v[i[2]]])
            .collect();

        let comp = v1[0].iter().sum();
        if v1.iter().all(|x| x.iter().sum::<u8>() == comp) {
            // Rotate the groups around until the lowest value
            // is at the top
            let min_v = v1.iter().map(|n| n[0]).min().unwrap();
            while v1[0][0] != min_v {
                let x = v1.pop().unwrap();
                v1.insert(0, x);
            }

            // Turn it into a string
            let mut ts = String::new();
            for n in v1.into_iter().flatten() {
                let s = format!("{}", n);
                ts.push_str(&s);
            }

            let b = ts.parse::<u64>().unwrap();

            if ts.len() <= limit && b > m {
                m = b;
            }
        }

        next_perm(&mut v);
    }

    return m
}

// This determines the indices of the three elements
// With an n-gon ring I'm counting all three outer 'extremes'
// as 0..x . The innards for lack of a better word will always
// start from x followed by the next one. The only problem is
// at the end where it has to wrap back around to x.
fn get_indices(x: usize) -> Vec<Vec<usize>> {
    (0..x)
        .map(|i| {
            let fi = i as usize;
            let fii = (i + x) as usize;
            let fiii = (((i + 1) % x) + x) as usize;

            vec![fi, fii, fiii]
        })
        .collect()
}

#[test]
fn test_n_gon_ring() {
    assert_eq!(n_gon_ring(3, usize::MAX), 432621513);
    assert_eq!(n_gon_ring(4, usize::MAX), 462723831516);
    assert_eq!(n_gon_ring(5, 16), 6531031914842725);
}
