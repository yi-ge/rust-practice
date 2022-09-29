// 字符串轮转
// https://leetcode.cn/problems/string-rotation-lcci
// INLINE  ../../images/string/string_rotation_lcci.jpeg
// 注意：并非最优方法。仅展示思路。

pub struct Solution;

impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        let n = s1.len();
        if s2.len() != n {
            return false;
        }

        if n == 0 {
            return true;
        }

        for i in 0..n {
            let mut flag = true;
            for j in 0..n {
                let m = (i + j) % n;
                if &s1[m..=m] != &s2[j..=j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                return true;
            }
        }

        false
    }
}
