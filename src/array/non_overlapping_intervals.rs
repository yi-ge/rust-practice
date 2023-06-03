// 无重叠区间
// https://leetcode.cn/problems/non-overlapping-intervals/
// INLINE  ../../images/array/non_overlapping_intervals.jpeg

pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // 如果输入为空，返回 0
        if intervals.is_empty() {
            return 0;
        }

        // 按照区间右端点排序
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        // 初始化结果为 1，因为第一个区间一定是不重叠的
        let mut res = 1;
        // 初始化起点为第一个区间的右端点
        let mut start = intervals[0][1];

        // 遍历剩余的区间
        for i in 1..intervals.len() {
            // 如果当前区间的左端点大于等于起点，说明不重叠
            if intervals[i][0] >= start {
                // 结果加 1
                res += 1;
                // 更新起点为当前区间的右端点
                start = intervals[i][1];
            }
        }

        // 总区间数减去不重叠的区间数就是需要移除的区间数
        intervals.len() as i32 - res
    }
}
