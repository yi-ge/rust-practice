// 与对应负数同时存在的最大正整数
// https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative
// INLINE  ../../images/sort/largest_positive_integer_that_exists_with_its_negative.jpeg

pub struct Solution;

impl Solution {
    // 找到与对应负数同时存在的最大正整数
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort(); // 对数组进行排序
        let mut max_k = -1; // 初始化最大正整数
        let mut i = 0; // 定义左指针
        let mut j = nums.len() - 1; // 定义右指针
        while i < j {
            // 当左指针小于右指针时
            if nums[i] + nums[j] == 0 {
                // 如果左指针所在的数与右指针所在的数相加等于零
                max_k = nums[j]; // 更新最大正整数为右指针所在的数
                break; // 跳出循环
            } else if nums[i] + nums[j] < 0 {
                // 如果左指针所在的数与右指针所在的数相加小于零
                i += 1; // 左指针向右移动一位
            } else {
                // 否则
                j -= 1; // 右指针向左移动一位
            }
        }
        max_k // 返回最大正整数
    }
}
