mod validator;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // start at the end, double every second number
    // if number is > 10, then subtract 9
    // replace the number at this position with this new number
    // sum all new digits, and return true if result is evenly divisible by 10
    let mut double = 0;
    let mut sum = 0;

    for ch in code.chars().rev() {
        let mut digit = ch.to_digit(10).unwrap();

        if double == 1 {
            digit *= 2;
            if digit > 9 {
                digit -= 9
            }
            double = 0;
        } else {
            double += 1;
        }

        sum += digit;
    }

    sum % 10 == 0
}
