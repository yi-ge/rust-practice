// 分割数组的最大值
// https://leetcode.cn/problems/split-array-largest-sum
// INLINE  ../../images/array/split_array_largest_sum.jpeg

pub struct Solution;

impl Solution {
    fn check(nums: &Vec<i32>, k: i32, x: i32) -> bool {
        let mut sum = 0;
        let mut cnt = 1;
        for num in nums {
            if sum + num > x {
                cnt += 1;
                sum = *num;
            } else {
                sum += num;
            }
        }
        cnt <= k
    }
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for num in &nums {
            left = left.max(*num);
            right += num;
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if Solution::check(&nums, k, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
