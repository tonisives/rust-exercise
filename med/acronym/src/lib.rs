pub fn abbreviate(phrase: &str) -> String {
    let mut acronym: Vec<char> = vec![];
    let mut prev_whitespace = true;
    let mut prev_lowercase = false;

    for c in phrase.chars() {
        if c == ' ' || c == '-' {
            prev_whitespace = true;
        } else {
            if prev_whitespace {
                acronym.push(c.to_ascii_uppercase());
                prev_whitespace = false;
                prev_lowercase = false;
                continue;
            }

            if c.is_uppercase() {
                if prev_lowercase {
                    acronym.push(c);
                }
                prev_lowercase = false
            } else if c.is_lowercase() {
                prev_lowercase = true;
            }
        }
    }

    acronym.into_iter().collect()
}
