pub fn egg_count(display_value: u32) -> usize {
    (0..(size_of_val(&display_value) * 8)).filter(|index| ((display_value >> index) & 1) == 1).count()
}
