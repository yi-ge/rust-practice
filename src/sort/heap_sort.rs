use crate::libs::heap::{build_heap_up_down, heapify_up_down};

pub fn heap_sort(nums: &mut Vec<i32>) {
    build_heap_up_down(nums); // 大顶堆

    for i in (0..nums.len()).rev() {
        nums.swap(0, i);
        heapify_up_down(nums, 0, i);
    }
}
