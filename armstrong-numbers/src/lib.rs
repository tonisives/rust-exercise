pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let total_chars = str.len();
    let chars = str.chars();
    let mut sum = 0;

    for char in chars {
        sum += char.to_digit(10).unwrap_or(0).pow(total_chars as u32)
    }

    sum == num
}
