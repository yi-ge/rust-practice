// 相等行列对
// https://leetcode.cn/problems/equal-row-and-column-pairs
// INLINE  ../../images/array/equal_row_and_column_pairs.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // 定义一个函数 equal_pairs，它接受一个二维整数向量并返回一个整数。
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        // 创建一个名为 count_map 的新 HashMap。
        // 键将是一个整数向量，值将是一个整数。
        // count_map 最初是空的。
        let mut count_map: HashMap<Vec<i32>, i32> = HashMap::new();

        // 遍历 grid 中的每一行。
        for row in &grid {
            // 将当前行插入 count_map 中（如果它尚未存在）。
            // 如果它已经在 count_map 中，则将其值加 1。
            // 这将计算每一行的出现次数。
            *count_map.entry(row.clone()).or_insert(0) += 1;
        }

        // 创建一个整数 res，并将其设置为 0。
        // 这将保存函数的结果。
        let mut res = 0;

        // 创建一个整数 n，并将其设置为 grid 的长度。
        // 这将用于遍历 grid 的列。
        let n = grid.len();

        // 遍历 grid 的列。
        for j in 0..n {
            // 创建一个名为 arr 的新向量。
            // 这将保存当前列的值。
            let mut arr = Vec::new();

            // 遍历 grid 的行。
            for i in 0..n {
                // 将当前列的值添加到 arr 中。
                arr.push(grid[i][j]);
            }

            // 如果 arr 在 count_map 中，则将 count_map[arr] 的值添加到 res 中。
            if let Some(&v) = count_map.get(&arr) {
                res += v;
            }
        }

        // 返回 res。
        res
    }
}
