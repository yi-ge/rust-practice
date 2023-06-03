// 选择排序
pub fn select_sort(mut nums: Vec<i32>) -> Vec<i32> {
    // 如果数组为空，直接返回
    if nums.is_empty() {
        return nums;
    }

    // 遍历整个数组
    for i in 0..nums.len() - 1 {
        // 假设当前元素为最小值的索引
        let mut index = i;
        // 在i之后的元素中找到最小的元素的索引
        for j in i + 1..nums.len() {
            if nums[j] < nums[index] {
                index = j;
            }
        }

        // 如果找到的最小元素不是当前元素，则交换位置
        if index != i {
            nums.swap(i, index);
        }
    }

    nums
}
