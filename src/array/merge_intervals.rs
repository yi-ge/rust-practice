// 合并区间
// https://leetcode.cn/problems/merge-intervals/
// INLINE  ../../images/array/merge_intervals.jpeg

pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 如果intervals数组长度小于2，则直接返回原数组
        if intervals.len() < 2 {
            return intervals;
        }
        // 对intervals数组按照左边界升序排序
        intervals.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 0..intervals.len() {
            let res_len = res.len();

            // 如果res数组为空或者当前区间的左边界大于res数组最后一个区间的右边界，
            // 则将当前区间加入res数组
            if res.is_empty() || res[res_len - 1][1] < intervals[i][0] {
                res.push(intervals[i].clone());
            } else {
                // 否则，更新res数组最后一个区间的右边界为当前区间右边界和res数组最后一个区间右边界中的较大值
                res[res_len - 1][1] = res[res_len - 1][1].max(intervals[i][1]);
            }
        }

        res
    }
}
