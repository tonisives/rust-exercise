pub fn abbreviate(phrase: &str) -> String {
    let mut acronym: Vec<char> = vec![];
    let mut prev_whitespace = true;

    for c in phrase.chars() {
        if c == ' ' || c == '-' {
            prev_whitespace = true;
        } else {
            if prev_whitespace {
                acronym.push(c.to_ascii_uppercase());
                prev_whitespace = false;
            } else if c.is_uppercase() {
                acronym.push(c)
            }
        }
    }

    acronym.into_iter().collect()
}
