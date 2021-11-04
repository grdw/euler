fn next_perm(res: &mut Vec<u8>) {
    // Assume res = [0,2,1]
    let mut i = res.len() - 1; // this becomes 2
    while res[i - 1] >= res[i] { // is 2 >= 1 YES!
        i -= 1 // this becomes 1
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

#[test]
fn test_next_perm() {
    let mut g = vec![0,1,2];
    next_perm(&mut g);
    assert_eq!(g, vec![0, 2, 1]);

    let mut g = vec![0,1,2];
    for _ in 0..2 {
        next_perm(&mut g);
    }
    assert_eq!(g, vec![1, 0, 2]);

    let mut g = vec![0,1,2];
    for _ in 0..5 {
        next_perm(&mut g);
    }
    assert_eq!(g, vec![2, 1, 0]);

    let mut group = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 0..1_000_000-1 {
        next_perm(&mut group);
    }
    assert_eq!(group, [2, 7, 8, 3, 9, 1, 5, 4, 6, 0])
}
