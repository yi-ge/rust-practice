// 单调数列
// https://leetcode.cn/problems/monotonic-array/
// INLINE  ../../images/array/monotonic_array.jpeg

pub struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut increasing = true; // 是否递增
        let mut diminishing = true; // 是否递减
        for i in 0..nums.len() - 1 {
            // 遍历数组
            if nums[i] < nums[i + 1] {
                // 如果当前元素小于后面的元素
                increasing = false; // 将递增标志位设为 false
            }

            if nums[i] > nums[i + 1] {
                // 如果当前元素大于后面的元素
                diminishing = false; // 将递减标志位设为 false
            }

            if !increasing && !diminishing {
                // 如果既不递增也不递减
                return false; // 数组不是单调的，返回 false
            }
        }

        increasing || diminishing // 如果是递增或递减，说明数组是单调的，返回 true
    }
}
