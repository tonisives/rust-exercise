pub fn square_of_sum(n: u32) -> u32 {
    let mut res = 0;
    for i in 1..=n {
        res += i
    }
    return res.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut res = 0;

    for i in 1..=n {
        res += i.pow(2)
    }

    res
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
