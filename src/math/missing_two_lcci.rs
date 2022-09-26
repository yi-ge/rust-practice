// 消失的两个数字
// https://leetcode.cn/problems/missing-two-lcci
// INLINE  ../../images/math/missing_two_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 + 2;
        let ab = (1..=n).chain(nums.iter().copied()).fold(0, |a, b| a ^ b);
        let l = ab & -ab;
        let (x, y) = (1..=n).chain(nums.into_iter()).fold((0, 0), |(x, y), v| {
            if v & l == 0 {
                (x ^ v, y)
            } else {
                (x, y ^ v)
            }
        });
        vec![x, y]
    }
}
