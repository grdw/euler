const MOD: usize = 1_000_000_007;

fn main() {
    let total_valid_castles =
        valid_castles(10_usize.pow(12), 100) +
        valid_castles(10_000, 10_000) +
        valid_castles(100, 10_usize.pow(12));

    println!("Answer: {}", total_valid_castles % MOD);
}

struct Row {
    y: usize,
    space: Vec<char>,
    block_count: usize,
}

fn valid_castles(width: usize, height: usize) -> usize {
    let row = Row { y: 0, space: vec!['1'; width], block_count: 1 };
    let mut valid_configurations = 0;
    generate_possibilities(&row, 0, height, &mut valid_configurations);
    return valid_configurations
}

fn generate_possibilities(
    row: &Row,
    block_count: usize,
    max: usize,
    valid_configurations: &mut usize) {

    // determine all the possibilities that can be stacked on top of
    // row
    // for all those possibilities make the next one's
    let rows = possibilities_for(row, max);
    if rows.is_empty() {
        return
    }

    if block_count % 2 == 0 {
        *valid_configurations += rows.len() as usize
    }

    for r in rows {
        generate_possibilities(
            &r,
            row.block_count + r.block_count,
            max,
            valid_configurations,
        )
    }
}

fn possibilities_for(row: &Row, max: usize) -> Vec<Row> {
    // if there's 'space' anywhere?
    // Do I need the full config or only all the 'top bits'; as in knowing
    // where there's space... because you could do:
    //
    // x
    // xxxxx
    //
    // x
    // xxxxx
    if row.y + 1 >= max {
        return vec![]
    }

    // How to generate:
    // 1000 1 block
    // 0100 1 block
    // 0010 1 block
    // 1100 1 block
    // 0011 1 block
    // 0101 2 blocks
    // 1010 2 blocks
    // 1110 1 block
    // 0111 1 block
    //
    return vec![
        Row { y: row.y + 1, space: vec!['1'; row.space.len()] }
    ];
}

#[test]
fn test_valid_castles() {
    assert_eq!(valid_castles(4, 2), 10);
    assert_eq!(valid_castles(13, 10), 3729050610636);
    assert_eq!(valid_castles(10, 13), 37959702514);
    assert_eq!(valid_castles(100, 100) % MOD, 841913936);
}
