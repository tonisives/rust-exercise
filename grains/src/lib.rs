pub fn square(s: u32) -> u64 {
    const BASE: u64 = 2;
    return BASE.pow(s - 1);
}

pub fn total() -> u64 {
    let mut total = 0;

    for i in 1..=64 {
        total += square(i);
    }

    total
}
