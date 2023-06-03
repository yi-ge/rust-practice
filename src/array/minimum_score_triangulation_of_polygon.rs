// 多边形三角剖分的最低得分
// https://leetcode.cn/problems/minimum-score-triangulation-of-polygon
// INLINE  ../../images/array/minimum_score_triangulation_of_polygon.jpeg
use std::cmp;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        // 初始化一个HashMap，用于存储已经计算过的(i,j)对应的最低得分
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        let n = values.len();

        // 定义dp函数，返回(i,j)对应的最低得分
        fn dp(
            i: usize,
            j: usize,
            values: &Vec<i32>,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            // 如果i和j的距离小于2，无法构成三角形，返回0
            if i + 2 > j {
                return 0;
            }
            // 如果i和j的距离等于2，直接计算并返回最低得分
            if i + 2 == j {
                return values[i] * values[i + 1] * values[j];
            }
            // 如果memo中已经有了(i,j)对应的最低得分，直接返回
            let key = (i, j);
            if !memo.contains_key(&key) {
                // 否则，遍历所有可能的k，计算(i,k,j)的得分加上dp(i,k)和dp(k,j)的得分，取最小值作为(i,j)的最低得分
                let mut min_score = i32::MAX;
                for k in (i + 1)..j {
                    min_score = cmp::min(
                        min_score,
                        values[i] * values[k] * values[j]
                            + dp(i, k, values, memo)
                            + dp(k, j, values, memo),
                    );
                }
                memo.insert(key, min_score);
            }
            *memo.get(&key).unwrap()
        }

        // 调用dp函数，计算(0,n-1)对应的最低得分，并返回
        dp(0, n - 1, &values, &mut memo)
    }
}