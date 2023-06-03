// 强整数
// https://leetcode.cn/problems/powerful-integers
// INLINE  ../../images/map/powerful_integers.jpeg

use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        // 创建一个 HashSet 存储结果
        let mut result = HashSet::new();
        // 初始化 value1 为 1
        let mut value1 = 1;
        // 迭代 x 的幂，最多迭代 21 次
        for _ in 0..21 {
            // 初始化 value2 为 1
            let mut value2 = 1;
            // 迭代 y 的幂，最多迭代 21 次
            for _ in 0..21 {
                // 计算两个幂的和
                let value = value1 + value2;
                // 如果和小于等于 bound，将其加入结果 HashSet 中
                if value <= bound {
                    result.insert(value);
                } else {
                    // 如果和大于 bound，跳出内层循环
                    break;
                }
                // 更新 value2 为 y 的下一个幂
                value2 *= y;
            }
            // 如果 value1 大于 bound，跳出外层循环
            if value1 > bound {
                break;
            }
            // 更新 value1 为 x 的下一个幂
            value1 *= x;
        }
        // 将结果 HashSet 转换为 Vec<i32> 并返回
        result.into_iter().collect()
    }
}
