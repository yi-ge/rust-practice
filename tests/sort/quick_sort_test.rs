use rust_practice::sort::quick_sort::quick_sort;

#[test]
fn test_quick_sort() {
    assert_eq!(
        quick_sort(vec![4, 2, 3, 1, 9, 6, 5, 8, 7]),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    assert_eq!(
        quick_sort(vec![1, 2, 3, 4, 9, 6, 5, 8, 7]),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    assert_eq!(quick_sort(vec![4, 3, 2, 1]), vec![1, 2, 3, 4]);

    assert_eq!(quick_sort(vec![1]), vec![1]);
    assert_eq!(quick_sort(vec![]), vec![]);
}
