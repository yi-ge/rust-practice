use rust_practice::libs::heap::{build_heap_down_up, build_heap_up_down, insert, remove_max};

#[test]
fn test_build_heap_down_up() {
    let mut nums = vec![1, 2, 3, 4, 5];
    build_heap_down_up(&mut nums);
    assert_eq!(nums, [5, 4, 2, 1, 3]);
}

#[test]
fn test_build_heap_up_down() {
    let mut nums = vec![1, 2, 3, 4, 5];
    build_heap_up_down(&mut nums);
    assert_eq!(nums, [5, 4, 3, 1, 2]);
}

#[test]
fn test_insert() {
    let mut nums = vec![5, 4, 2, 1, 3];
    insert(&mut nums, 6);
    assert_eq!(nums, [6, 4, 5, 1, 3, 2]);
}

#[test]
fn test_remove_max() {
    let mut nums = vec![5, 4, 2, 1, 3];
    remove_max(&mut nums);
    assert_eq!(nums, [4, 3, 2, 1]);
}
