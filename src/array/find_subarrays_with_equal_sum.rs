// 和相等的子数组
// https://leetcode.cn/problems/find-subarrays-with-equal-sum
// INLINE  ../../images/array/find_subarrays_with_equal_sum.jpeg

pub struct Solution;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut sums: Vec<(usize, usize, i32)> = Vec::new();

        // 计算所有长度为 2 的子数组的和，并保存子数组的起始和结束位置
        for i in 0..n - 1 {
            sums.push((i, i + 1, nums[i] + nums[i + 1]));
        }

        // 检查是否存在两个子数组具有相同的和且起始位置不同
        for i in 0..sums.len() {
            for j in i + 1..sums.len() {
                if sums[i].2 == sums[j].2 && sums[i].0 != sums[j].0 {
                    return true;
                }
            }
        }

        false
    }
}
