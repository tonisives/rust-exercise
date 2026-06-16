// return which prime number index it is, if is a prime number
pub fn nth(n: u32) -> u32 {
    // let i = 0;
    // find prime numbers <= n
    // get the index of this prime number (if exists in this array)

    // how to find prime number? divide by all integers until sqrt(n)
    let mut idx: u32 = 0;
    let mut i = 0;

    loop {
        if is_prime(i) {
            if idx == n {
                return i;
            }
            idx += 1;
        }
        i += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}
