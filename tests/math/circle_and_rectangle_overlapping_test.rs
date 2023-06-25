use rust_practice::math::circle_and_rectangle_overlapping::Solution;

#[test]
fn check_overlap() {
    // 示例 1 ：
    // 输入：radius = 1, xCenter = 0, yCenter = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1
    // 输出：true
    // 解释：圆和矩形存在公共点 (1,0) 。
    let radius = 1;
    let x_center = 0;
    let y_center = 0;
    let x1 = 1;
    let y1 = -1;
    let x2 = 3;
    let y2 = 1;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        true
    );

    // 示例 2 ：
    // 输入：radius = 1, xCenter = 1, yCenter = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1
    // 输出：false
    let radius = 1;
    let x_center = 1;
    let y_center = 1;
    let x1 = 1;
    let y1 = -3;
    let x2 = 2;
    let y2 = -1;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        false
    );

    // 示例 3 ：
    // 输入：radius = 1, xCenter = 0, yCenter = 0, x1 = -1, y1 = 0, x2 = 0, y2 = 1
    // 输出：true
    let radius = 1;
    let x_center = 0;
    let y_center = 0;
    let x1 = -1;
    let y1 = 0;
    let x2 = 0;
    let y2 = 1;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        true
    );
}
