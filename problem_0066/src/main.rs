//Consider quadratic Diophantine equations of the form:
// x^2 – Dy^2 = 1
// For example, when D=13, the minimal solution in x is 649^2 – 13×180^2 = 1.
//
//
// Step 1: I rewrote the function in the form of:
//
// D = (x^2 - 1) / y^2
fn main() {
    println!("The answer to euler 66 is: {:?}", problem_0066(1000))
}

fn problem_0066(max: i64) -> i64 {
    let mut max_x: i64 = 0;
    
    for d in 1..=max {
        if (d as f32).sqrt().fract() == 0.0 {
            continue
        }

        println!("Finding a value for d: {:?}", d);
        let (x, _y) = diophantine_equation(d);
        if x > max_x { 
            max_x = x
        }
    }

    max_x
}

#[test]
fn test_highest_x() {
    assert_eq!(problem_0066(7), 9);
    assert_eq!(problem_0066(100), 9);
}

fn diophantine_equation(d: i64) -> (i64, i64) {

    let mut x: i64 = 1;
    let mut y: i64 = 0;

    'outer: loop {
        x += 1;
        loop {
            y += 1;

            let td: f64 = (x.pow(2) - 1) as f64 / y.pow(2) as f64;
            println!("{:?} {:?} {:?}", x, y, d);
            if td < d as f64 {
                y = 0;
                break 
            } else if td == d as f64 {
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
    assert_eq!(diophantine_equation(13), (649, 180));
    assert_eq!(diophantine_equation(61), (649, 180))
}
