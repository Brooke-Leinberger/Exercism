pub fn collatz(n: u64) -> Option<u64> {
    if n == 1 {
        return Some(0);
    }

    if n % 2 == 0 {
        return Some(1 + collatz(n/2).unwrap());
    }

    else {
        return Some(1 + collatz(n*3 + 1).unwrap());
    }
}