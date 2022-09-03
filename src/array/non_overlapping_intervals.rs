// 无重叠区间
// https://leetcode.cn/problems/non-overlapping-intervals/
// INLINE  ../../images/array/non_overlapping_intervals.jpeg

pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut res = 1;
        let mut start = intervals[0][1];

        for i in 1..intervals.len() {
            if intervals[i][0] >= start {
                res += 1;
                start = intervals[i][1];
            }
        }

        intervals.len() as i32 - res
    }
}
