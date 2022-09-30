// 零矩阵
// https://leetcode.cn/problems/zero-matrix-lcci
// INLINE  ../../images/array/zero_matrix_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut flag_col0 = false; // 标记第0行是否出现0
        for i in 0..m {
            if matrix[i][0] == 0 {
                flag_col0 = true;
            }

            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in (0..=m - 1).rev() {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }

            if flag_col0 {
                matrix[i][0] = 0;
            }
        }
    }
}
