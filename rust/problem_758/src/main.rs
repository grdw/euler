fn main() {
    println!("Hello, world!");
}

// https://projecteuler.net/problem=758
// Run with 'cargo test'

fn pourable(l: &(i32, i32), r: &(i32, i32)) -> bool {
    l.1 > 1 && (r.0 - r.1) > 0
}

fn pour(
    l: &mut (i32, i32),
    r: &mut (i32, i32)) {

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
    };
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
        pour(&mut l, &mut s);
    }

    if pourable(&m, &s) && !picked {
        //println!("M -> S");
        picked = true;
        pour(&mut m, &mut s);
    };

    if pourable(&s, &m) && !picked {
        //println!("S -> M");
        picked = true;
        pour(&mut s, &mut m);
    };

    if pourable(&l, &m) && !picked{
        //println!("L -> M");
        picked = true;
        pour(&mut l, &mut m);
    };

    if pourable(&m, &l) && !picked{
        //println!("M -> L");
        picked = true;
        pour(&mut m, &mut l);
    };

    if pourable(&s, &l) && !picked {
        //println!("S -> L");
        pour(&mut s, &mut l);
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
    let mut small_bucket = (3, 3);
    let mut medium_bucket = (5, 0);
    pour(&mut small_bucket, &mut medium_bucket);

    assert_eq!(small_bucket.1, 0);
    assert_eq!(medium_bucket.1, 3);
}

#[test]
fn test_pour_half() {
    let mut medium_bucket = (5, 5);
    let mut small_bucket = (2, 0);
    pour(&mut medium_bucket, &mut small_bucket);

    assert_eq!(medium_bucket.1, 3);
    assert_eq!(small_bucket.1, 2);
}

#[test]
fn test_pour_precise() {
    let mut small_bucket = (3, 3);
    let mut medium_bucket = (5, 3);
    pour(&mut small_bucket, &mut medium_bucket);

    assert_eq!(small_bucket.1, 1);
    assert_eq!(medium_bucket.1, 5);
}

#[test]
fn test_pour_half_part_two() {
    let mut medium_bucket = (5, 5);
    let mut small_bucket = (8, 3);
    pour(&mut medium_bucket, &mut small_bucket);

    assert_eq!(medium_bucket.1, 0);
    assert_eq!(small_bucket.1, 8);
}

#[test]
fn test_pour_half_part_three() {
    let mut medium_bucket = (8, 5);
    let mut small_bucket = (3, 0);
    pour(&mut medium_bucket, &mut small_bucket);

    assert_eq!(medium_bucket.1, 2);
    assert_eq!(small_bucket.1, 3);
}

#[test]
#[should_panic]
fn test_pour_when_r_bucket_no_space() {
    let mut small_bucket = (3, 3);
    let mut medium_bucket = (5, 5);
    pour(&mut small_bucket, &mut medium_bucket);
}

#[test]
#[should_panic]
fn test_pour_when_l_bucket_empty() {
    let mut small_bucket = (3, 0);
    let mut medium_bucket = (5, 3);
    pour(&mut small_bucket, &mut medium_bucket);
}
