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
    if input.len() == 0 {
        return HashMap::new();
    }
    let mut res: HashMap<char, usize> = HashMap::new();
    let chunks = input.chunks(
        input
            .len()
            .div_ceil(worker_count),
    );

    let handles: Vec<_> = chunks
        .map(|chunk| spawn_counter(chunk))
        .collect();

    handles
        .into_iter()
        .for_each(|handle| {
            if let Ok(map) = handle.join() {
                add_to_map(map, &mut res);
            }
        });

    res
}

fn add_to_map(from: HashMap<char, usize>, into: &mut HashMap<char, usize>) {
    from.iter()
        .for_each(|(char, size)| {
            *into
                .entry(*char)
                .or_insert(0) += size
        })
}

fn spawn_counter(input: &[&str]) -> JoinHandle<HashMap<char, usize>> {
    let copy: Vec<String> = input
        .iter()
        .map(|c| c.to_string())
        .collect();

    spawn(move || {
        let mut res: HashMap<char, usize> = HashMap::new();

        copy.iter()
            .for_each(|str| add_chars(str, &mut res));

        res
    })
}

fn add_chars(str: &str, map: &mut HashMap<char, usize>) {
    str.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .for_each(|char| {
            *map.entry(char)
                .or_insert(0) += 1;
        })
}
