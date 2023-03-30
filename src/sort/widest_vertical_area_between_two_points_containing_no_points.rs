// 两点之间不包含任何点的最宽垂直区域
// https://leetcode.cn/problems/widest-vertical-area-between-two-points-containing-no-points
// INLINE  ../../images/sort/widest_vertical_area_between_two_points_containing_no_points.jpeg

pub struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[0].cmp(&b[0]));
        (1..points.len()).fold(0, |ret, i| ret.max(points[i][0] - points[i - 1][0]))
    }
}
