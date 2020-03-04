fn is_triplet(x: i32, y: i32, z: i32) -> bool {
    if y < x || z < y {
        return false
    }

    let powers = (x.pow(2) + y.pow(2)) as f32;
    let result = powers.sqrt();

    result == z as f32
}

fn find_triplet(total: i32) -> (i32, i32, i32) {
    let (mut x, mut y, mut z) = (1, 1, total - 2);

    loop {
        if is_triplet(x, y, z) {
            println!("TRIPLET FOUND");
            break
        }

        println!("{}, {}, {}", x, y, z);

        if y < z {
            y += 1;
            z -= 1;
        } else if x < y {
            x += 1;
            y -= 1;
        } else {
            x = -1;
            y = -1;
            z = -1;
            break;
        }
    }

    (x, y, z)
}

#[test]
fn pythagorean_triplet() {
    assert_eq!(is_triplet(3,4,5), true);
    assert_eq!(is_triplet(3,4,6), false);
    assert_eq!(is_triplet(6,8,10), true);
    assert_eq!(is_triplet(48,55,73), true);
    assert_eq!(is_triplet(17,144,145), true);
}

#[test]
fn find_triplet_test() {
    assert_eq!(find_triplet(12), (3, 4, 5));
    //assert_eq!(find_triplet(1000), (0, 0, 0));
}
