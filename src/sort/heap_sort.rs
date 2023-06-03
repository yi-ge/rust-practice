// 导入堆的库
use crate::libs::heap::{build_heap_up_down, heapify_up_down};

// 堆排序函数
pub fn heap_sort(nums: &mut Vec<i32>) {
    build_heap_up_down(nums); // 构建大顶堆

    // 从数组末尾开始遍历
    for i in (0..nums.len()).rev() {
        nums.swap(0, i); // 将最大值放到数组末尾
        heapify_up_down(nums, 0, i); // 对剩余部分重新构建大顶堆
    }
}
