// 和等于 k 的最长子数组长度
// https://leetcode.cn/problems/maximum-size-subarray-sum-equals-k/
// INLINE  ../../images/array/maximum_size_subarray_sum_equals_k.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut res = 0;
        let mut map = HashMap::with_capacity(nums.len());
        map.insert(0, -1); // 最长的情况是从0开始所有元素

        for i in 0..nums.len() {
            sum += nums[i];

            // 题目要的是最长
            if !map.contains_key(&sum) {
                map.insert(sum, i as i32);
            }

            if let Some(&v) = map.get(&(sum - k)) {
                res = res.max(i as i32 - v);
            }
        }

        res
    }
}
