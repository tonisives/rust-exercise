pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let line_amount = take_down * 4;
    let mut res: Vec<String> = Vec::with_capacity((line_amount + take_down - 1) as usize);

    for i in 0..take_down {
        let from = start_bottles - i;
        let is_last_verse = i == take_down - 1; // new lines

        let lines = lines(from, is_last_verse);
        lines.into_iter().for_each(|line| res.push(line));
        if !is_last_verse {
            res.push("\n".to_string())
        }
    }

    res.join("")
}

fn lines(bottles: u32, is_last_verse: bool) -> [String; 4] {
    let as_str = num_as_str(bottles);
    [
        format!(
            "{} green {} hanging on the wall,\n",
            as_str,
            bottles_plural(bottles)
        ),
        format!(
            "{} green {} hanging on the wall,\n",
            as_str,
            bottles_plural(bottles)
        ),
        format!("{}", "And if one green bottle should accidentally fall,\n"),
        last_line(bottles, is_last_verse),
    ]
}

fn last_line(bottles: u32, is_last_verse: bool) -> String {
    if bottles == 1 {
        "There'll be no green bottles hanging on the wall.".to_string()
    } else {
        let new_line = if is_last_verse { "" } else { "\n" }.to_string();
        format!(
            "There'll be {} green {} hanging on the wall.{}",
            num_as_str(bottles - 1).to_lowercase(),
            bottles_plural(bottles - 1),
            new_line
        )
    }
}

fn bottles_plural(num: u32) -> String {
    if num > 1 { "bottles" } else { "bottle" }.to_string()
}

fn num_as_str(num: u32) -> String {
    match num {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        1 => "One",
        _ => "Unknown",
    }
    .to_string()
}
