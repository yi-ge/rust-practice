// 字符串轮转
// https://leetcode.cn/problems/string-rotation-lcci
// INLINE  ../../images/string/string_rotation_lcci.jpeg
// 注意：并非最优方法。仅展示思路。

pub struct Solution;

impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        let n = s1.len(); // 获取字符串s1的长度
        if s2.len() != n {
            // 如果字符串s2长度不等于s1的长度
            return false; // 返回false
        }

        if n == 0 {
            // 如果s1的长度为0
            return true; // 返回true
        }

        for i in 0..n {
            // 遍历s1的长度
            let mut flag = true; // 定义flag为true
            for j in 0..n {
                // 遍历s1的长度
                let m = (i + j) % n; // 计算出m的值
                if &s1[m..=m] != &s2[j..=j] {
                    // 如果s1[m]不等于s2[j]
                    flag = false; // 将flag设置为false
                    break; // 跳出循环
                }
            }
            if flag {
                // 如果flag为true
                return true; // 返回true
            }
        }

        false // 否则返回false
    }
}
