pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut current_prime: u64 = 2; // always prime
    let mut current_num = n as f64;

    loop {
        current_num = current_num as f64 / current_prime as f64;

        if current_num.fract() == 0.0 {
            factors.push(current_num as u64);
            if current_num == current_prime as f64 {
                factors.push(current_num as u64);
                break;
            }
            current_prime = 2;
        }

        current_prime = next_prime(current_prime);
    }

    factors
}

// TODO: optimize?
fn next_prime(n: u64) -> u64 {
    let mut next = n;

    loop {
        next += 1;
        if is_prime(next) {
            return next;
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i <= n / i {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}
