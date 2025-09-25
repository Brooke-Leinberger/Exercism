pub fn square(s: u32) -> u64 {
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    // Clever way would be square(65) - 1, but square(65) = u64::MAX + 1, so causes a panic
    // Using u64::MAX feels like cheating

    let mut fin = 0;
    for i in 1..65 {
        fin += square(i);
    }

    fin
}
