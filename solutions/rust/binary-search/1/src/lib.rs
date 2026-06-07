pub fn find(array: &[i32], key: i32) -> Option<usize> {
    array.binary_search_by_key(&key, |&value| value).ok()
}
