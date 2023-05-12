// 与对应负数同时存在的最大正整数
// https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative
// INLINE  ../../images/sort/largest_positive_integer_that_exists_with_its_negative.jpeg

pub struct Solution;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut max_k = -1;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            if nums[i] + nums[j] == 0 {
                max_k = nums[j];
                break;
            } else if nums[i] + nums[j] < 0 {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_k
    }
}
