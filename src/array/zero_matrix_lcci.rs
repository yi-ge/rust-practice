// 零矩阵
// https://leetcode.cn/problems/zero-matrix-lcci
// INLINE  ../../images/array/zero_matrix_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len(); // 矩阵行数
        let n = matrix[0].len(); // 矩阵列数
        let mut flag_col0 = false; // 标记第0行是否出现0
        for i in 0..m {
            if matrix[i][0] == 0 {
                // 如果第i行第0列为0，标记第0行
                flag_col0 = true;
            }

            for j in 1..n {
                if matrix[i][j] == 0 {
                    // 如果第i行第j列为0，将第0行第j列和第i行第0列标记为0
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in (0..=m - 1).rev() {
            // 从后往前遍历，避免覆盖
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    // 如果第i行第0列或第0行第j列为0，将第i行第j列设为0
                    matrix[i][j] = 0;
                }
            }

            if flag_col0 {
                // 如果第0行出现0，将第0列设为0
                matrix[i][0] = 0;
            }
        }
    }
}
