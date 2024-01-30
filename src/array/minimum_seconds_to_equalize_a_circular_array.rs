// 使循环数组所有元素相等的最少秒数
// https://leetcode.cn/problems/minimum-seconds-to-equalize-a-circular-array
// INLINE  ../../images/array/minimum_seconds_to_equalize_a_circular_array.jpeg

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        let n = nums.len();
        let mut res = n;

        for (i, num) in nums.iter().enumerate() {
            map.entry(*num).or_insert(Vec::new()).push(i);
        }

        for pos in map.values() {
            let mut mx = pos[0] + n - pos[pos.len() - 1];
            for i in 1..pos.len() {
                mx = mx.max(pos[i] - pos[i - 1]);
            }
            res = res.min(mx / 2);
        }

        res as i32
    }
}
