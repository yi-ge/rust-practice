use rust_practice::graphs::reorder_routes_to_make_all_paths_lead_to_the_city_zero::Solution;

#[test]
fn min_reorder() {
    // 示例 1：
    // 输入：n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
    // 输出：3
    // 解释：更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。
    let n = 6;
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    assert_eq!(Solution::min_reorder(n, connections), 3);

    // 示例 2：
    // 输入：n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
    // 输出：2
    // 解释：更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。
    let n = 5;
    let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
    assert_eq!(Solution::min_reorder(n, connections), 2);

    // 示例 3：
    // 输入：n = 3, connections = [[1,0],[2,0]]
    // 输出：0
    let n = 3;
    let connections = vec![vec![1, 0], vec![2, 0]];
    assert_eq!(Solution::min_reorder(n, connections), 0);
}
