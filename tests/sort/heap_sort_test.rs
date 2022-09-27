use rust_practice::sort::heap_sort::heap_sort;

#[test]
fn test_heap_sort() {
    let mut list = vec![4, 2, 3, 1, 9, 6, 5, 8, 7];
    heap_sort(&mut list);
    assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
