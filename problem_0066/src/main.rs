//Consider quadratic Diophantine equations of the form:
// x^2 – Dy^2 = 1
// For example, when D=13, the minimal solution in x is 649^2 – 13×180^2 = 1.
fn main() {
    println!("The answer to euler 66 is: {:?}", problem_0066())
}

fn problem_0066() -> i32 {
    0
}

fn diophantine_equation(d: i32) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    'outer: loop {
        x += 1;
        loop {
            y += 1;

            if (d as i64 * (y as i64).pow(2)) >= i32::MAX as i64 {
                y = 0;
                break
            }

            if x.pow(2) - (d * y.pow(2)) == 1 {
                break 'outer
            }
        }
    }
    (x, y)
}

#[test]
fn test_minimal_solution() {
    assert_eq!(diophantine_equation(2), (3, 2));
    assert_eq!(diophantine_equation(3), (2, 1));
    assert_eq!(diophantine_equation(5), (9, 4));
    assert_eq!(diophantine_equation(6), (5, 2));
    assert_eq!(diophantine_equation(7), (8, 3));
    assert_eq!(diophantine_equation(13), (649, 180))
}
