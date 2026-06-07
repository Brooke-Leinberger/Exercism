pub fn egg_count(display_value: u32) -> usize {
    let bits = 0..(size_of_val(&display_value) * 8);
    bits.filter(|index| ((display_value >> index) & 1) == 1).count()
}
