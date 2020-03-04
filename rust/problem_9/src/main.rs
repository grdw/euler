fn is_triplet(x: i32, y: i32, z: i32) -> bool {
    if y < x || z < y {
        return false
    }

    let powers = (x.pow(2) + y.pow(2)) as f32;
    let result = powers.sqrt();

    result == z as f32
}

fn find_triplet(total: i32) -> (i32, i32, i32) {
    let (mut x, mut y, mut z) = (1, 1, 1);

    'x: loop {
        if x >= total {
            break 'x;
        }

        loop {
            if y >= total {
                y = 1;
                break
            }

            loop {
                if z >= total {
                    z = 1;
                    break
                }

                if x + y + z == total && is_triplet(x, y, z) {
                    break 'x;
                }

                z += 1;
            }
            y += 1;
        }
        x += 1;
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
    assert_eq!(find_triplet(24), (6, 8, 10));
    assert_eq!(find_triplet(176), (48, 55, 73));
    assert_eq!(find_triplet(1000), (200, 375, 425));
}
