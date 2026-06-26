pub fn raindrops(n: u32) -> String {
    // 3, 5 and 7
    let sound = format!(
        "{}{}{}",
        if n % 3 == 0 { "Pling" } else { "" },
        if n % 5 == 0 { "Plang" } else { "" },
        if n % 7 == 0 { "Plong" } else { "" }
    );

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}
