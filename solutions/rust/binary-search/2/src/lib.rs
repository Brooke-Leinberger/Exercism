pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() { return None }
    
    let mut min = 0;
    let mut max = array.len() - 1;

    while max - min > 2
    {
        let mid = (min + max) / 2;
        if array[mid] == key { return Some(mid); }
        if key > array[mid] { min = mid }
        else { max = mid }
    }

    if array[min] == key { return Some(min); }
    if array[max] == key { return Some(max); }
    None
}
