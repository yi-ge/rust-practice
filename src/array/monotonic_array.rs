// 单调数列
// https://leetcode.cn/problems/monotonic-array/
// INLINE  ../../images/array/monotonic_array.jpeg

pub struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut increasing = true;
        let mut diminishing = true;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] {
                increasing = false;
            }

            if nums[i] > nums[i + 1] {
                diminishing = false;
            }

            if !increasing && !diminishing {
                return false;
            }
        }

        increasing || diminishing
    }
}
