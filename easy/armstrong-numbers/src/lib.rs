pub fn is_armstrong_number(num: u32) -> bool {
    is_armstrong_number_map(num)
}

pub fn is_armstrong_number_mine(num: u32) -> bool {
    let str = num.to_string();
    let total_chars = str.len();
    let chars = str.chars();
    let mut sum = 0;

    for char in chars {
        sum += char.to_digit(10).unwrap_or(0).pow(total_chars as u32)
    }

    sum == num
}

pub fn is_armstrong_number_map(num: u32) -> bool {
    let str = num.to_string();
    let len = str.len();
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(len as u32))
        .sum::<u32>()
        == num
}
