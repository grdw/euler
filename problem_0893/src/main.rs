fn main() {
    println!("Answer: {}", total_min_matchsticks(10_000_000));
}

fn total_min_matchsticks(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += min_matchsticks(i)
    }
    return total
}

#[test]
fn test_total_min_matchsticks() {
    assert_eq!(total_min_matchsticks(100), 916);
}

fn min_matchsticks(n: u64) -> u64 {
    let mut lowest = string_to_matchstick_number(n);
    let p = find_lowest_partition(n, lowest);
    if p < lowest {
        println!("{} {:?} {:?} {}", n, divisors(n), p, lowest);
        lowest = p
    }

    for (l, r) in divisors(n) {
        let lc = string_to_matchstick_number(l);
        let rc = string_to_matchstick_number(r);
        let t = lc + rc + to_matchsticks('x');
        if t < lowest {
            //println!("{}, {}, {} -> {}", n, l, r, t);
            lowest = t;
        }
    }
    lowest
}

fn find_lowest_partition(n: u64, lowest: u64) -> u64 {
    let mut all_partitions = vec![];
    let mut current_partition = vec![];
    find_partitions(n, n, lowest, &mut current_partition, &mut all_partitions);
    println!("============= {}", n);
    for p in &all_partitions {
        println!("{:?}", p);
    }
    return 1000
}



fn find_partitions(
    n: u64,
    max_val: u64,
    lowest: u64,
    current_partition: &mut Vec<(u64, u64)>,
    all_partitions: &mut Vec<Vec<(u64, u64)>>) {

    let mut t: u64 = current_partition
        .iter()
        .map(|(a, b)| string_to_matchstick_number(*a) * b)
        .sum();
    let c: u64 = current_partition.iter().map(|(_, b)| b).sum();
    if c > 0 {
        t += (c - 1) * to_matchsticks('+');
    }

    if n == 0 && t < lowest {
        all_partitions.push(current_partition.clone());
        return;
    }

    for i in (1..=max_val.min(n)).rev() {
        if let Some(last) = current_partition.last_mut() {
            if last.0 == i {
                last.1 += 1;
            } else {
                current_partition.push((i, 1));
            }
        } else {
            current_partition.push((i, 1));
        }

        find_partitions(n - i, i, lowest, current_partition, all_partitions);

        if let Some(last) = current_partition.last_mut() {
            if last.1 > 1 {
                last.1 -= 1;
            } else {
                current_partition.pop();
            }
        }
    }
}


#[test]
fn test_min_matchsticks() {
    assert_eq!(min_matchsticks(1), 2);
    assert_eq!(min_matchsticks(2), 5);
    assert_eq!(min_matchsticks(28), 9)
}

fn string_to_matchstick_number(n: u64) -> u64 {
    let s = n.to_string();
    return s.chars().map(|c| to_matchsticks(c)).sum();
}

fn divisors(n: u64) -> Vec<(u64, u64)> {
    let mut list = vec![];
    let max = (n as f64).sqrt().round() as u64;
    for i in 2..=max {
        if n % i == 0 {
            list.push((i, n / i))
        }
    }
    return list
}

fn to_matchsticks(n: char) -> u64 {
    match n {
        '1' | '+' | 'x' => 2,
        '7' => 3,
        '4' => 4,
        '2' | '3' | '5' => 5,
        '6' | '9' | '0' => 6,
        '8' => 7,
        _ => panic!("Invalid character")
    }
}
