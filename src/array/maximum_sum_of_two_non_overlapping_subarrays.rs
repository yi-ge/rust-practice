// 两个非重叠子数组的最大和
// https://leetcode.cn/problems/maximum-sum-of-two-non-overlapping-subarrays
// INLINE  ../../images/array/maximum_sum_of_two_non_overlapping_subarrays.jpeg

pub struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        let mut res = 0;
        for i in 0..=(n - (first_len + second_len) as usize) {
            for j in (i + first_len as usize)..=(n - second_len as usize) {
                res = res.max(
                    prefix_sum[i + first_len as usize] - prefix_sum[i]
                        + prefix_sum[j + second_len as usize]
                        - prefix_sum[j],
                );
            }
        }
        for i in 0..=(n - (first_len + second_len) as usize) {
            for j in (i + second_len as usize)..=(n - first_len as usize) {
                res = res.max(
                    prefix_sum[i + second_len as usize] - prefix_sum[i]
                        + prefix_sum[j + first_len as usize]
                        - prefix_sum[j],
                );
            }
        }
        res
    }
}
