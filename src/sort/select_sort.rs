pub fn select_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    for i in 0..nums.len() - 1 {
        let mut index = i;
        for j in i + 1..nums.len() {
            // 找出i之后元素中最小的元素的索引
            if nums[j] < nums[index] {
                index = j;
            }
        }

        if index != i {
            nums.swap(i, index);
        }
    }

    nums
}
