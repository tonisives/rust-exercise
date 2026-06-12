/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // start at the end, double every second number
    // if number is > 10, then subtract 9
    // replace the number at this position with this new number
    // sum all new digits, and return true if result is evenly divisible by 10
    let mut double = 0;
    let mut sum = 0;
    let mut invalid = false;
    let mut digit_count = 0;

    for ch in code.chars().rev() {
        if ch == ' ' {
            continue;
        }

        let mut digit = match ch.to_digit(10) {
            Some(digit) => digit,
            None => {
                invalid = true;
                break;
            }
        };

        digit_count += 1;

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

    if invalid || (sum == 0 && digit_count == 1) {
        false
    } else {
        sum % 10 == 0
    }
}
