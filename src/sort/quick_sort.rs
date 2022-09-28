pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len <= 1 {
        return nums;
    }
    quick_sort_recursion(&mut nums, 0, len - 1);
    nums
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right]; // 基准值。最好选择随机一个元素，否则可能碰到极端情况。

    let mut i = left;
    // 默认不包含right，因此pivot取right更方便
    for j in left..right {
        if nums[j] < pivot {
            nums.swap(i, j);
            i += 1;
        }
    }

    nums.swap(right, i);
    i
}

fn quick_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot = partition(nums, left, right);
    if pivot != 0 {
        quick_sort_recursion(nums, left, pivot - 1);
    }
    quick_sort_recursion(nums, pivot + 1, right);
}
