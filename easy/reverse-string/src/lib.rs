pub fn reverse_simple(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn reverse(input: &str) -> String {
    // 2 allocations needed for unicode
    let mut chars = Vec::with_capacity(input.len());

    for c in input.chars() {
        chars.push(c);
    }

    // in-place half array swap
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1); // safe sub 
    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }

    chars.iter().collect()
}
