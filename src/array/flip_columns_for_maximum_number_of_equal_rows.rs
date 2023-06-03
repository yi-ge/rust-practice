// 按列翻转得到最大值等行数
// https://leetcode.cn/problems/flip-columns-for-maximum-number-of-equal-rows
// INLINE  ../../images/array/flip_columns_for_maximum_number_of_equal_rows.jpeg

pub struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        // 初始化结果数和哈希表
        let mut res = 0;
        let mut map = std::collections::HashMap::new();
        // 遍历矩阵的每一行
        for row in matrix {
            // 复制一份当前行
            let mut row = row;
            // 如果当前行的首位为0，就将当前行按位取反
            if row[0] == 0 {
                for i in 0..row.len() {
                    row[i] = 1 - row[i];
                }
            }
            // 将当前行转换成字符串并作为哈希表的键，计算键的出现次数
            let key = row
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            let count = map.entry(key).or_insert(0);
            *count += 1;
            // 更新结果数为当前出现次数和原结果数的较大值
            res = res.max(*count);
        }
        // 返回结果数
        res
    }
}
