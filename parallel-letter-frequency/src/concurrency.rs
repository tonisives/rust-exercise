use std::collections::HashMap;
use tokio::{runtime, task::JoinSet};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(worker_count)
        .build()
        .unwrap();

    runtime.block_on(run_frequency(input, worker_count));

    HashMap::new()
}

async fn run_frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks = input.chunks(
        input
            .len()
            .div_ceil(worker_count.max(1)),
    );

    let mut set = JoinSet::new();
    for chunk in chunks {
        set.spawn(async move { count(chunk) });
    }

    HashMap::new()
}

fn count(chunk: &[&str]) -> HashMap<char, usize> {
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
