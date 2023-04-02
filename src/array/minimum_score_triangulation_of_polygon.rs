// 多边形三角剖分的最低得分
// https://leetcode.cn/problems/minimum-score-triangulation-of-polygon
// INLINE  ../../images/array/minimum_score_triangulation_of_polygon.jpeg
use std::cmp;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        let n = values.len();

        fn dp(
            i: usize,
            j: usize,
            values: &Vec<i32>,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if i + 2 > j {
                return 0;
            }
            if i + 2 == j {
                return values[i] * values[i + 1] * values[j];
            }
            let key = (i, j);
            if !memo.contains_key(&key) {
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

        dp(0, n - 1, &values, &mut memo)
    }
}
