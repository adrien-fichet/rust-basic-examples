fn main() {
    array_last_3_elements();
}

fn array_last_3_elements() {
    let array = [1, 2, 3, 4, 5];
    assert_eq!(array.get(array.len().wrapping_sub(3)), Some(&3));
    assert_eq!(array.get(array.len().wrapping_sub(2)), Some(&4));
    assert_eq!(array.last(), Some(&5));
}
