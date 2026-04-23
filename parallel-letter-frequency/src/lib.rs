use std::{
    collections::HashMap,
    thread::{JoinHandle, spawn},
};

// todo!(
//     "Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
//     match worker_count {
//         1 => "1 worker".to_string(),
//         _ => format!("{worker_count} workers"),
//     }
// );
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut res: HashMap<char, usize> = HashMap::new();
    let chunks = input.chunks(worker_count);

    let handles: Vec<_> = chunks
        .map(|chunk| spawn_counter(chunk).join())
        .collect();
    handles
        .iter()
        .for_each(|handle| match handle {
            Ok(handle_ok) => handle_ok
                .iter()
                .for_each(|(char, size)| {
                    *res.entry(*char)
                        .or_insert(0) += size
                }),
            Err(_) => (),
        });

    res
}

fn spawn_counter(input: &[&str]) -> JoinHandle<HashMap<char, usize>> {
    let copy: Vec<String> = input
        .iter()
        .map(|c| c.to_string())
        .collect();

    spawn(move || {
        let mut res: HashMap<char, usize> = HashMap::new();

        copy.iter()
            .for_each(|str| {
                str.chars()
                    .filter(|c| c.is_alphabetic())
                    .flat_map(|c| c.to_lowercase())
                    .for_each(|char| {
                        *res.entry(char)
                            .or_insert(0) += 1;
                    })
            });

        res
    })
}
