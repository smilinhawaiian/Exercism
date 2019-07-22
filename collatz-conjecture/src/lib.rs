pub fn collatz(n: u64) -> Option<u64> {
    let mut count: u64 = 0;
    if n < 1 {
        return None;
    } else if n > 1 {
        if n % 2 == 0 {
            // even
            count = collatz(n / 2).unwrap() + 1;
        } else {
            // odd
            count = collatz(3 * n + 1).unwrap() + 1;
        }
    }
    Some(count)
}
