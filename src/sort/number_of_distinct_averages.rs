// 不同的平均值数目
// https://leetcode.cn/problems/number-of-distinct-averages
// INLINE  ../../images/sort/number_of_distinct_averages.jpeg

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        // 先排序
        let mut nums = nums;
        nums.sort();

        // 使用set保存结果
        let mut res_set: HashSet<i32> = HashSet::new();

        // 从左右两边开始移动
        for (i, j) in (0..nums.len()).zip((0..nums.len()).rev()) {
            res_set.insert(nums[i] + nums[j]);
        }
        res_set.len() as i32
    }
}
