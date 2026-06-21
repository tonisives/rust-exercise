pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut divisor = 2; // always prime
    let mut n = n;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }

        divisor += 1;
    }

    factors
}
