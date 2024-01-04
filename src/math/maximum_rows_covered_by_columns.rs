// 被列覆盖的最多行数
// https://leetcode.cn/problems/maximum-rows-covered-by-columns
// INLINE  ../../images/math/maximum_rows_covered_by_columns.jpeg

pub struct Solution;

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut mask = vec![0; m];
        for i in 0..m {
            for j in 0..n {
                mask[i] += matrix[i][j] << (n - j - 1);
            }
        }

        let mut res = 0;
        let mut cur: u32 = 0;
        let limit = 1 << n;
        while (cur + 1) < limit {
            cur += 1;
            if cur.count_ones() != num_select as u32 {
                continue;
            }
            let mut t = 0;
            for j in 0..m {
                if (mask[j] as u32 & cur) == mask[j] as u32 {
                    t += 1;
                }
            }
            res = res.max(t);
        }

        res
    }
}
