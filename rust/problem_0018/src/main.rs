fn main() {
    println!("Hello, world!");
}

fn problem_18(mut triangle: Vec<Vec<u32>>) -> u32 {
    let mut inbetween = triangle.pop().unwrap();

    for layer in triangle.iter_mut().rev() {
        for (index, value) in layer.iter_mut().enumerate() {
            let x1 = inbetween.get(index).unwrap();
            let x2 = inbetween.get(index + 1).unwrap_or(&0);

            if *value + x1 > *value + x2 {
                *value += x1
            } else {
                *value += x2
            }
        }

        inbetween = layer.to_vec();
    }

    triangle[0][0]
}

#[test]
fn test_triangle() {
    let triangle = vec![
        vec![3],
        vec![7, 4],
        vec![2, 4, 6],
        vec![8, 5, 9, 3]
    ];

    assert_eq!(problem_18(triangle), 23);
}

#[test]
fn test_big_triangle() {
    let big_triangle = vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 04, 82, 47, 65],
        vec![19, 01, 23, 75, 03, 34],
        vec![88, 02, 77, 73, 07, 63, 67],
        vec![99, 65, 04, 28, 06, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]
    ];

    assert_eq!(problem_18(big_triangle), 1074);
}
