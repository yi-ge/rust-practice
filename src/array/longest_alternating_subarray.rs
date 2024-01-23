// 最长交替子数组
// https://leetcode.cn/problems/longest-alternating-subarray
// INLINE  ../../images/array/longest_alternating_subarray.jpeg

pub struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let n = nums.len();
        for first_index in 0..n {
            for i in (first_index + 1)..n {
                let length = i - first_index + 1;
                if nums[i] - nums[first_index]
                    == TryInto::<i32>::try_into((length - 1) % 2).unwrap()
                {
                    res = res.max(length as i32);
                } else {
                    break;
                }
            }
        }
        res
    }
}
