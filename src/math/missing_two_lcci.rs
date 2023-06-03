// 消失的两个数字
// https://leetcode.cn/problems/missing-two-lcci
// INLINE  ../../images/math/missing_two_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        // 数组长度比缺失的数字个数多两个，因此总数字个数为 n = nums.len() + 2
        let n = nums.len() as i32 + 2;
        // 计算总数字异或上原数组中的数字的结果，即为缺失数字异或的结果
        let ab = (1..=n).chain(nums.iter().copied()).fold(0, |a, b| a ^ b);
        // 找到缺失数字二进制下最低位为 1 的位，将缺失数字分为两组，分别与原数组中的数字异或，即可得到两个缺失数字
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
