fn is_square(n: u64) -> bool {
    let root = (n as f64).sqrt() as u64;
    root * root == n
}

fn solve_pell(d: u64) -> (u64, u64) {
    let mut p = 0;
    let mut q = 1;
    let mut a0 = (d as f64).sqrt() as u64;
    let mut a = a0;
    let mut m = 0;
    let mut n = 1;

    loop {
        if q == 0 {
            continue; // skip this convergent
        }
        m = a * q - p;
        n = (d - m * m) / q;
        a = (a0 + m) / n;
        let new_p = p;
        p = a * p + 1 * q;
        q = new_p;
        if p * p - d * q * q == 1 {
            return (p, q);
        }
    }
}

fn main() {
    let mut max_x = 0;
    let mut max_d = 0;

    for d in 2..=1000 {
        if is_square(d) {
            continue;
        }

        println!("{:?}", d);
        let (x, _) = solve_pell(d);
        if x > max_x {
            max_x = x;
            max_d = d;
        }
    }

    println!("Maximum value of x is {} when D is {}", max_x, max_d);
}
