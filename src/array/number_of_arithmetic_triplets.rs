// 算术三元组的数目
// https://leetcode.cn/problems/number-of-arithmetic-triplets
// INLINE  ../../images/array/number_of_arithmetic_triplets.jpeg
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        // 创建一个 Hashset 存储 nums 中的元素
        let mut hash_set = HashSet::new();
        for &i in &nums {
            hash_set.insert(i);
        }
        // 记录答案的变量
        let mut ans = 0;
        // 遍历 nums 中的每个元素
        for &i in &nums {
            // 如果存在 i+diff 和 i+2*diff，则找到了一个符合条件的算术三元组
            if hash_set.contains(&(i + diff)) && hash_set.contains(&(i + 2 * diff)) {
                ans += 1;
            }
        }
        // 返回答案
        ans
    }
}
