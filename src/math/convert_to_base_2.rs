// 负二进制转换
// https://leetcode.cn/problems/convert-to-base-2
// INLINE  ../../images/math/convert_to_base_2.jpeg

pub struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut n = n;
        let mut result = String::new();
        while n != 0 {
            let remainder = n % -2;
            n /= -2;
            if remainder < 0 {
                n += 1;
            }
            result.push_str(&remainder.abs().to_string());
        }
        if result.is_empty() {
            result.push_str("0");
        }
        result.chars().rev().collect()
    }
}
