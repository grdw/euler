fn main() {
    println!("Answer: {}", detract_all(10_u128.pow(17)));
}

fn detract_all(n: u128) -> u128 {
    let comb = all_cubes(n);
    let mut abacus = vec![0; comb.len()];
    let mut steps = 0;

    loop {
        let mut total = 0;
        let mut j = 0;
        abacus[0] += 1;

        for m in 0..comb.len() {
            total += abacus[m] * comb[m];
            if m + 1 < comb.len() && total >= comb[m + 1] {
                j = m + 1;
            }
        }

        for m in 0..j {
            abacus[m] = 0;
        }

        if j > 0 {
            abacus[j] += 1;
        }

        if total > n {
            break;
        }

        for s in &abacus {
            steps += s;
        }
        println!("{:?} {}", abacus, total);

    }
    return steps
}

#[test]
fn test_detract_all() {
    assert_eq!(detract_all(100), 512);
}

fn all_cubes(n: u128) -> Vec<u128> {
    let mut results = vec![];
    let mut m: u128 = 1;
    loop {
        let p = m.pow(3);
        if p > n {
            break;
        }

        results.push(p);
        m += 1;
    }
    return results;
}
