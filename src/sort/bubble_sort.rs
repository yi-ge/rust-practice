/**
 * 冒泡排序
 * @param nums 待排序的数组
 * @return 排序后的数组
 */
pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return nums;
    }

    for i in 0..nums.len() - 1 {
        let mut flag = false; // 是否发生交换
        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                // 如果前一个数比后一个数大，则交换两个数的位置
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
                flag = true; // 标记发生交换
            }
        }
        if !flag {
            return nums; // 如果没有数据交换提前结束排序
        }
    }

    nums
}
