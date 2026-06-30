use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    // limit: level
    // factors: points

    let mut points: HashSet<u32> = HashSet::new();

    for &factor in factors {
        if factor == 0 || factor >= limit {
            continue;
        }
        let mut mult = factor;

        while mult < limit {
            points.insert(mult);
            mult = mult + factor;
        }
    }

    points.iter().sum()
}
