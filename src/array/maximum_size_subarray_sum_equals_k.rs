// 和等于 k 的最长子数组长度
// https://leetcode.cn/problems/maximum-size-subarray-sum-equals-k/
// INLINE  ../../images/array/maximum_size_subarray_sum_equals_k.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0; // 当前的和
        let mut res = 0; // 最长子数组长度
        let mut map = HashMap::with_capacity(nums.len()); // 记录和以及对应的下标

        map.insert(0, -1); // 最长的情况是从0开始所有元素，因此和为0的下标为-1

        for i in 0..nums.len() {
            sum += nums[i];

            // 如果map中没有当前的和，则将和以及对应的下标插入到map中
            if !map.contains_key(&sum) {
                map.insert(sum, i as i32);
            }

            // 如果map中有和为(sum - k)的值，则说明当前子数组的和为k，计算当前子数组的长度
            if let Some(&v) = map.get(&(sum - k)) {
                res = res.max(i as i32 - v);
            }
        }

        res
    }
}
