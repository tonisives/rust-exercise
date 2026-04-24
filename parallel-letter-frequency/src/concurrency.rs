use std::collections::HashMap;
use tokio::{runtime, task::JoinSet};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(worker_count.max(1))
        .build()
        .unwrap();

    runtime.block_on(run_frequency(input, worker_count))
}

async fn run_frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let mut set = JoinSet::new();
    let chunks = input.chunks(
        input
            .len()
            .div_ceil(worker_count.max(1)),
    );

    // spawn jobs into set
    for chunk in chunks {
        // note: we could use join_all for no copy, but that runs on single thread
        let owned: Vec<String> = chunk
            .iter()
            .map(|s| s.to_string())
            .collect();

        set.spawn(async move { count(owned) });
    }

    // wait jobs in set with while
    while let Some(res) = set
        .join_next()
        .await
    {
        if let Ok(map) = res {
            for (c, n) in map {
                *result
                    .entry(c)
                    .or_insert(0) += n;
            }
        }
    }
    result
}

fn count(chunk: Vec<String>) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    for s in chunk {
        for c in s
            .chars()
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase())
        {
            *res.entry(c)
                .or_insert(0) += 1;
        }
    }

    res
}
