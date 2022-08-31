// 两数之和
// https://leetcode.cn/problems/two-sum
// INLINE  ../../images/array/two_sum.jpeg

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let ans = target - nums[i];
            if map.contains_key(&ans) {
                return vec![i as i32, map[&ans] as i32];
            }

            map.insert(nums[i], i);
        }

        vec![]
    }
}
