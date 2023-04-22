// 出现最频繁的偶数元素
// https://leetcode.cn/problems/most-frequent-even-element
// INLINE  ../../images/array/most_frequent_even_element.jpeg

pub struct Solution;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;
        let mut cnt = BTreeMap::new();
        nums.into_iter().for_each(|num| {
            if num % 2 == 0 {
                *cnt.entry(num).or_default() += 1
            }
        });
        cnt.into_iter()
            .rev()
            .max_by_key(|&e| e.1)
            .unwrap_or((-1, -1))
            .0
    }
}
