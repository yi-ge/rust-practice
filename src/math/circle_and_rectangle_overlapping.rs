// 圆和矩形是否有重叠
// https://leetcode.cn/problems/circle-and-rectangle-overlapping
// INLINE  ../../images/math/circle_and_rectangle_overlapping.jpeg

pub struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x = if x_center < x1 {
            x1
        } else if x_center > x2 {
            x2
        } else {
            x_center
        };
        let y = if y_center < y1 {
            y1
        } else if y_center > y2 {
            y2
        } else {
            y_center
        };
        (x - x_center) * (x - x_center) + (y - y_center) * (y - y_center) <= radius * radius
    }
}
