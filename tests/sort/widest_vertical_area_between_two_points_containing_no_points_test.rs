use rust_practice::sort::widest_vertical_area_between_two_points_containing_no_points::Solution;

#[test]
fn max_width_of_vertical_area() {
    // 示例 1：
    // 输入：points = [[8,7],[9,9],[7,4],[9,7]]
    // 输出：1
    // 解释：红色区域和蓝色区域都是最优区域。
    let points1 = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
    assert_eq!(Solution::max_width_of_vertical_area(points1), 1);

    // 示例 2：
    // 输入：points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
    // 输出：3
    let points2 = vec![
        vec![3, 1],
        vec![9, 0],
        vec![1, 0],
        vec![1, 4],
        vec![5, 3],
        vec![8, 8],
    ];
    assert_eq!(Solution::max_width_of_vertical_area(points2), 3);
}
