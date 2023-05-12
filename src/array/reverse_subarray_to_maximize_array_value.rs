// 翻转子数组得到最大的数组值
// https://leetcode.cn/problems/reverse-subarray-to-maximize-array-value
// INLINE  ../../images/array/reverse_subarray_to_maximize_array_value.jpeg
// 题目描述较难理解。

pub struct Solution;

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut d = 0;
        let mut mx = i32::MIN;
        let mut mn = i32::MAX;
        let n = nums.len();

        for i in 1..n {
            let a = nums[i - 1];
            let b = nums[i];
            base += (a - b).abs();
            mx = mx.max(a.min(b));
            mn = mn.min(a.max(b));
            d = d.max(
                (nums[0] - b).abs() - (a - b).abs().max((nums[n - 1] - a).abs() - (a - b).abs()),
            );
        }

        base + d.max(2 * (mx - mn))
    }
}
