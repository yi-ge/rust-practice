// 按列翻转得到最大值等行数
// https://leetcode.cn/problems/flip-columns-for-maximum-number-of-equal-rows
// INLINE  ../../images/array/flip_columns_for_maximum_number_of_equal_rows.jpeg

pub struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut map = std::collections::HashMap::new();
        for row in matrix {
            let mut row = row;
            if row[0] == 0 {
                for i in 0..row.len() {
                    row[i] = 1 - row[i];
                }
            }
            let key = row
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            let count = map.entry(key).or_insert(0);
            *count += 1;
            res = res.max(*count);
        }
        res
    }
}
