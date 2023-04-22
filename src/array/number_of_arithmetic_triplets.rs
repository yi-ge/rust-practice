// 算术三元组的数目
// https://leetcode.cn/problems/number-of-arithmetic-triplets
// INLINE  ../../images/array/number_of_arithmetic_triplets.jpeg
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut hash_set = HashSet::new();
        for &i in &nums {
            hash_set.insert(i);
        }
        let mut ans = 0;
        for &i in &nums {
            if hash_set.contains(&(i + diff)) && hash_set.contains(&(i + 2 * diff)) {
                ans += 1;
            }
        }
        ans
    }
}
