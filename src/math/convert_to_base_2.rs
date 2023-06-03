// 负二进制转换
// https://leetcode.cn/problems/convert-to-base-2
// INLINE  ../../images/math/convert_to_base_2.jpeg

pub struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        // 初始化变量n和结果字符串result
        let mut n = n;
        let mut result = String::new();

        // 当n不为0时，进行循环
        while n != 0 {
            // 求n对-2的余数
            let remainder = n % -2;
            // 将n除以-2
            n /= -2;
            // 如果余数小于0，则将n加1
            if remainder < 0 {
                n += 1;
            }
            // 将余数的绝对值转换为字符串并添加到结果字符串中
            result.push_str(&remainder.abs().to_string());
        }
        // 如果结果字符串为空，则添加字符0
        if result.is_empty() {
            result.push_str("0");
        }
        // 将结果字符串反转后返回
        result.chars().rev().collect()
    }
}
