fn main() {
    println!("Hello, world!");
}

// https://projecteuler.net/problem=758
// Run with 'cargo test'

fn pourable(l: &(i32, i32), r: &(i32, i32)) -> bool {
    l.1 > 1 && (r.0 - r.1) > 0
}

fn pour(mut l: (i32, i32), mut r: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    if !pourable(&l, &r) {
        panic!("illegal pour; left bucket empty or right bucket is full");
    }

    // A bucket is filled from L -> R.
    // Water free on the right: (r.0 - r.1)
    if (r.0 - r.1) < l.1 {
        l.1 = l.1 - (r.0 - r.1);
        r.1 = r.0;
    } else {
        r.1 = r.1 + l.1;
        l.1 = 0;
    }

    (l, r)
}

// The format of a bucket is (capacity, current liters of water)
fn pourings(
    mut s: (i32, i32),
    mut m: (i32, i32),
    mut l: (i32, i32),
    tries: i32
) -> i32 {

    if s.1 == 1 || m.1 == 1 || l.1 == 1 {
        return tries
    }

    // This is arbitrary for now
    if tries > 100 {
        panic!("OVERFLOW ----");
    }

    let mut picked = false;

    //println!("{} {} {} | ({})", s.1, m.1, l.1, tries);

    // Does S have capacity for the contents of M or L
    if pourable(&l, &s) && !picked {
        //println!("L -> S");
        picked = true;
        let (l1, s1) = pour(l, s);
        l = l1;
        s = s1;
    }

    if pourable(&m, &s) && !picked {
        //println!("M -> S");
        picked = true;
        let(m1, s1) = pour(m, s);
        m = m1;
        s = s1;
    };

    if pourable(&s, &m) && !picked {
        //println!("S -> M");
        picked = true;
        let(s1, m1) = pour(s, m);
        s = s1;
        m = m1;
    };

    if pourable(&l, &m) && !picked{
        //println!("L -> M");
        picked = true;
        let(l1, m1) = pour(l, m);
        l = l1;
        m = m1;
    };

    if pourable(&m, &l) && !picked{
        //println!("M -> L");
        picked = true;
        let(m1, l1) = pour(m, l);
        m = m1;
        l = l1;
    };

    if pourable(&s, &l) && !picked {
        //println!("S -> L");
        let(s1, l1) = pour(s, l);
        s = s1;
        l = l1;
    };

    pourings(s, m, l, tries + 1)
}

#[test]
fn pourings_for_1_liter() {
    let small_bucket = (3, 3);
    let medium_bucket = (5, 5);
    let large_bucket = (8, 0);

    let x = pourings(small_bucket, medium_bucket, large_bucket, 0);
    assert_eq!(x, 4);
}

#[test]
fn test_pour_full() {
    let small_bucket = (3, 3);
    let medium_bucket = (5, 0);
    let (l, r) = pour(small_bucket, medium_bucket);

    assert_eq!(l.1, 0);
    assert_eq!(r.1, 3);
}

#[test]
fn test_pour_half() {
    let medium_bucket = (5, 5);
    let small_bucket = (2, 0);
    let (l, r) = pour(medium_bucket, small_bucket);

    assert_eq!(l.1, 3);
    assert_eq!(r.1, 2);
}

#[test]
fn test_pour_precise() {
    let small_bucket = (3, 3);
    let medium_bucket = (5, 3);
    let (l, r) = pour(small_bucket, medium_bucket);

    assert_eq!(l.1, 1);
    assert_eq!(r.1, 5);
}

#[test]
fn test_pour_half_part_two() {
    let medium_bucket = (5, 5);
    let small_bucket = (8, 3);
    let (l, r) = pour(medium_bucket, small_bucket);

    assert_eq!(l.1, 0);
    assert_eq!(r.1, 8);
}

#[test]
fn test_pour_half_part_three() {
    let medium_bucket = (8, 5);
    let small_bucket = (3, 0);
    let (l, r) = pour(medium_bucket, small_bucket);

    assert_eq!(l.1, 2);
    assert_eq!(r.1, 3);
}

#[test]
#[should_panic]
fn test_pour_when_r_bucket_no_space() {
    let small_bucket = (3, 3);
    let medium_bucket = (5, 5);
    pour(small_bucket, medium_bucket);
}

#[test]
#[should_panic]
fn test_pour_when_l_bucket_empty() {
    let small_bucket = (3, 0);
    let medium_bucket = (5, 3);
    pour(small_bucket, medium_bucket);
}
