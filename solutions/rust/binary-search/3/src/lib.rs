pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let enums : Vec<(usize, &i32)> = array.iter().enumerate().collect();
    let mut slice = &enums[0..];
    
    while slice.len() > 2
    {
        let mix = slice.len() / 2;
        slice = if *slice[mix].1 < key { &slice[mix..] } else { &slice[..mix+1] }
    }

    // Should be at most 2 elements
    for remain in slice {if *remain.1 == key {return Some(remain.0)}}
    None
}
