pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let line_amount = take_down * 4;
    let mut res: Vec<String> = Vec::with_capacity(line_amount as usize);

    for i in 0..take_down {
        let from = start_bottles - i;
        let to = from - 1;

        let lines = lines(to);
        lines.into_iter().for_each(|line| res.push(line));
    }

    String::new()
}

fn lines(bottles: u32) -> [String; 4] {
    let as_str = num_as_str(bottles);
    [
        format!("{} green bottles hanging on the wall,", as_str),
        format!("{} green bottles hanging on the wall,", as_str),
        format!("{}", "And if one green bottle should accidentally fall,"),
        last_line(bottles),
    ]
}

fn last_line(bottles: u32) -> String {
    if bottles == 1 {
        "There'll be no green bottles hanging on the wall.".to_string()
    } else {
        format!(
            "There'll be {} green bottles hanging on the wall.",
            num_as_str(bottles - 1)
        )
    }
}

fn num_as_str(num: u32) -> &'static str {
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
}
