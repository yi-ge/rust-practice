// 二进制矩阵中的特殊位置
// https://leetcode.cn/problems/special-positions-in-a-binary-matrix
// INLINE  ../../images/array/special_positions_in_a_binary_matrix.jpeg
// 解题思路：参见官方题解2及注释

pub struct Solution;

impl Solution {
    pub fn num_special(mut mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        for i in 0..n {
            let mut row_count = 0;

            // 遍历行，统计该行出现1的次数
            for j in 0..m {
                if mat[i][j] == 1 {
                    row_count += 1;
                }
            }

            // * 这里是最难理解的。本来是要用第0行作为每一列有多少个1的计数，因此计数前本应该对该行先清0，再记录该行对应列出现了几次1，由于只对有1的列做计数，直接减去1避开了清0操作
            if i == 0 {
                row_count -= 1;
            }

            // 如果该行出现1的次数 > 0
            if row_count > 0 {
                // 遍历行，得到1所在的列，对出现大于1的列做累计计数
                for j in 0..m {
                    if mat[i][j] == 1 {
                        mat[0][j] += row_count;
                    }
                }
            }
        }

        let mut res = 0;
        for i in 0..m {
            if mat[0][i] == 1 {
                res += 1;
            }
        }

        res
    }
}
