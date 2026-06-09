// each empty square should show the number of mines nearby
pub fn annotate(garden: &[&str]) -> Vec<String> {
    println!("{garden:#?}");

    // solution 1: loop every character, if is not a mine, then loop all nearby nodes, and sum the
    // answer to the current position
    let mut res: Vec<String> = vec![]; // TOD: use arr

    for i in 0..garden.len() {
        let row = garden[i].as_bytes();
        let mut mapped_row = vec![0u8; row.len()];

        for j in 0..row.len() {
            if row[j] == b'*' {
                mapped_row[j] = b'*'
            } else {
                mapped_row[j] = find_nearby_count(i, j, garden)
            }
        }

        res.push(String::from_utf8(mapped_row).expect("not valid utf8"));
    }

    res
}

#[rustfmt::skip]
const DIRECTIONS: [(i64, i64); 8] = [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0),          (1, 0),
    (-1, -1), (0, -1), (1, -1)
];

fn find_nearby_count(i: usize, j: usize, garden: &[&str]) -> u8 {
    // don't go left/top/right/bottom if on the edge. otherwise, loop neighbours and count the mines

    let mut count = 0;

    for d in DIRECTIONS {
        if i == 0 && d.1 == 1 {
            // don't go up
            continue;
        }

        if i == garden.len() - 1 && d.1 == -1 {
            // don't go down
            continue;
        }

        if j == 0 && d.0 == -1 {
            // don't go left
            continue;
        }

        if j == garden[0].len() - 1 && d.0 == 1 {
            // don't go right
            continue;
        }

        println!("i:{i}, j:{j}, d{d:?}");
        if garden[(i as i64 + d.1) as usize].as_bytes()[(j as i64 + d.0) as usize] == b'*' {
            count += 1;
        }
    }

    count
}
