// 加一
// https://leetcode.cn/problems/plus-one/
// INLINE  ../../images/array/plus_one.jpeg

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        // 增加mut，因为需要修改digits数组
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                // 如果当前数字小于9，直接加一后返回
                digits[i] += 1;
                return digits;
            }

            // 如果当前数字等于9，将其置0，然后往左看一位
            digits[i] = 0;
            if i > 0 {
                i -= 1; // 往左看一位
            } else if i == 0 {
                // 如果已经看到最左边了，说明所有数字都是9，跳出循环
                break;
            }
        }

        // 能跳出循环说明所有数字都是9，需要进位
        digits = vec![0; digits.len() + 1]; // 创建一个新的数组，长度加1，全部置0
        digits[0] = 1; // 最高位设为1
        return digits;
    }
}
