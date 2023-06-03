// 两数之和
// https://leetcode.cn/problems/two-sum
// INLINE  ../../images/array/two_sum.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 创建一个HashMap，用于存储数值和它们对应的索引
        let mut map: HashMap<i32, usize> = HashMap::new();

        // 遍历数组
        for i in 0..nums.len() {
            // 计算目标数与当前数之差
            let ans = target - nums[i];
            // 如果HashMap中存在这个差值，说明找到了两个数
            if map.contains_key(&ans) {
                // 返回这两个数的索引
                return vec![i as i32, map[&ans] as i32];
            }

            // 将当前数和它的索引插入HashMap中
            map.insert(nums[i], i);
        }

        // 如果没有找到两个数，则返回一个空的Vec
        vec![]
    }
}
