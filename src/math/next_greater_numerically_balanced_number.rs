// 下一个更大的数值平衡数
// https://leetcode.cn/problems/next-greater-numerically-balanced-number
// INLINE  ../../images/math/next_greater_numerically_balanced_number.jpeg

pub struct Solution;

impl Solution {
    pub fn is_balanced_number(x: i32) -> bool {
        let mut count = vec![0; 10];
        let mut x = x;
        while x > 0 {
            count[(x % 10) as usize] += 1;
            x /= 10;
        }
        for d in 0..10 {
            if count[d] > 0 && count[d] != d {
                return false;
            }
        }
        true
    }

    pub fn next_beautiful_number(n: i32) -> i32 {
        for i in n + 1..=1224444 {
            if Self::is_balanced_number(i) {
                return i;
            }
        }
        -1
    }
}
