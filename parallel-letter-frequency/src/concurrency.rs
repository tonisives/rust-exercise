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
    for slice in chunks {
        let chunk = slice
            .iter()
            .map(|it| it.to_string())
            .collect();
        set.spawn(async move { count(chunk) });
    }

    // wait jobs in set with while
    while let Some(handle) = set
        .join_next()
        .await
    {
        if let Ok(res) = handle {
            for (c, count) in res {
                *result
                    .entry(c)
                    .or_insert(0) += count;
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
