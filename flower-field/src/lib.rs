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

fn find_nearby_count(i: usize, j: usize, garden: &[&str]) -> u8 {
    // don't go left/top/right/bottom if on the edge. otherwise, loop neighbours and count the mines
    //

    b'*'
}
