// 实现快速排序算法
pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len <= 1 {
        return nums;
    }
    quick_sort_recursion(&mut nums, 0, len - 1); // 调用递归函数进行快速排序
    nums // 返回排序后的数组
}

// 分区函数，用于确定基准值的位置，并将数组分为左右两部分
fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right]; // 基准值。最好选择随机一个元素，否则可能碰到极端情况。

    let mut i = left;
    // 默认不包含right，因此pivot取right更方便
    for j in left..right {
        if nums[j] < pivot {
            // 比基准值小的元素和i位置的元素进行交换
            nums.swap(i, j);
            i += 1;
        }
    }

    nums.swap(right, i); // 将基准值放到正确的位置
    i // 返回基准值的位置
}

// 快速排序递归函数
fn quick_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        // 如果左右指针相遇，则返回
        return;
    }

    let pivot = partition(nums, left, right); // 分区，确定基准值的位置
    if pivot != 0 {
        // 递归处理左半部分
        quick_sort_recursion(nums, left, pivot - 1);
    }
    quick_sort_recursion(nums, pivot + 1, right); // 递归处理右半部分
}
