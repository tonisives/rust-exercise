pub fn raindrops(n: u32) -> String {
    // 3, 5 and 7
    let pling = if n % 3 == 0 { "Pling" } else { "" };
    let plang = if n % 5 == 0 { "Plang" } else { "" };
    let plong = if n % 7 == 0 { "Plong" } else { "" };

    if pling == "" && plang == "" && plong == "" {
        return format!("{n}");
    }

    format!("{pling}{plang}{plong}")
}
