// 加一
// https://leetcode.cn/problems/plus-one/
// INLINE  ../../images/array/plus_one.jpeg

pub struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        // 增加mut
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                // 小于9直接+1返回
                digits[i] += 1;
                return digits;
            }

            // 大于9置0后进位
            digits[i] = 0;
            if i > 0 {
                i -= 1; // 往左看一位
            } else if i == 0 {
                // 全部数字都是9，先跳出循环
                break;
            }
        }

        // 能跳出循环说明全部数字都是9
        digits = vec![0; digits.len() + 1]; // 重置数组进位
        digits[0] = 1;
        return digits;
    }
}
