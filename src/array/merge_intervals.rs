// 合并区间
// https://leetcode.cn/problems/merge-intervals/
// INLINE  ../../images/array/merge_intervals.jpeg

pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() < 2 {
            return intervals;
        }
        intervals.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 0..intervals.len() {
            let res_len = res.len();

            if res.is_empty() || res[res_len - 1][1] < intervals[i][0] {
                res.push(intervals[i].clone());
            } else {
                res[res_len - 1][1] = res[res_len - 1][1].max(intervals[i][1]);
            }
        }

        res
    }
}
