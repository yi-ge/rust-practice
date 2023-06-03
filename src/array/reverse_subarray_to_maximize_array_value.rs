// 翻转子数组得到最大的数组值
// https://leetcode.cn/problems/reverse-subarray-to-maximize-array-value
// INLINE  ../../images/array/reverse_subarray_to_maximize_array_value.jpeg
// 题目描述较难理解。

pub struct Solution;

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut base = 0; // 基础值，即翻转前数组的值
        let mut d = 0; // 差值
        let mut mx = i32::MIN; // 最大值
        let mut mn = i32::MAX; // 最小值
        let n = nums.len(); // 数组长度

        // 遍历数组，计算基础值和最大最小值
        for i in 1..n {
            let a = nums[i - 1];
            let b = nums[i];
            base += (a - b).abs();
            mx = mx.max(a.min(b));
            mn = mn.min(a.max(b));
            // 计算差值
            d = d.max(
                (nums[0] - b).abs() - (a - b).abs().max((nums[n - 1] - a).abs() - (a - b).abs()),
            );
        }

        // 返回翻转后得到的最大值
        base + d.max(2 * (mx - mn))
    }
}
