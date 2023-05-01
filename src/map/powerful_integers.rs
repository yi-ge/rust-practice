// 强整数
// https://leetcode.cn/problems/powerful-integers
// INLINE  ../../images/map/powerful_integers.jpeg

use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut result = HashSet::new();
        let mut value1 = 1;
        for _ in 0..21 {
            let mut value2 = 1;
            for _ in 0..21 {
                let value = value1 + value2;
                if value <= bound {
                    result.insert(value);
                } else {
                    break;
                }
                value2 *= y;
            }
            if value1 > bound {
                break;
            }
            value1 *= x;
        }
        result.into_iter().collect()
    }
}
