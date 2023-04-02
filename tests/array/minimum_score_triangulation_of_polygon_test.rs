use rust_practice::array::minimum_score_triangulation_of_polygon::Solution;

#[test]
fn min_score_triangulation() {
    // 示例 1：
    // 输入：values = [1,2,3]
    // 输出：6
    // 解释：多边形已经三角化，唯一三角形的分数为 6。
    let values1 = vec![1, 2, 3];
    assert_eq!(Solution::min_score_triangulation(values1), 6);

    // 示例 2：
    // 输入：values = [3,7,4,5]
    // 输出：144
    // 解释：有两种三角剖分，可能得分分别为：3*7*5 + 4*5*7 = 245，或 3*4*5 + 3*4*7 = 144。最低分数为 144。
    let values2 = vec![3, 7, 4, 5];
    assert_eq!(Solution::min_score_triangulation(values2), 144);

    // 示例 3：
    // 输入：values = [1,3,1,4,1,5]
    // 输出：13
    // 解释：最低分数三角剖分的得分情况为 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13。
    let values3 = vec![1, 3, 1, 4, 1, 5];
    assert_eq!(Solution::min_score_triangulation(values3), 13);
}
