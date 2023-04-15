use rust_practice::graphs::flower_planting_with_no_adjacent::Solution;

#[test]
fn garden_no_adj() {
    // 示例 1：
    // 输入：n = 3, paths = [[1,2],[2,3],[3,1]]
    // 输出：[1,2,3]
    // 解释：
    // 花园 1 和 2 花的种类不同。
    // 花园 2 和 3 花的种类不同。
    // 花园 3 和 1 花的种类不同。
    // 因此，[1,2,3] 是一个满足题意的答案。其他满足题意的答案有 [1,2,4]、[1,4,2] 和 [3,2,1]
    let n = 3;
    let paths = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
    assert_eq!(Solution::garden_no_adj(n, paths), vec![1, 2, 3]);

    // 示例 2：
    // 输入：n = 4, paths = [[1,2],[3,4]]
    // 输出：[1,2,1,2]
    let n = 4;
    let paths = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(Solution::garden_no_adj(n, paths), vec![1, 2, 1, 2]);

    // 示例 3：
    // 输入：n = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
    // 输出：[1,2,3,4]
    let n = 4;
    let paths = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 1],
        vec![1, 3],
        vec![2, 4],
    ];
    assert_eq!(Solution::garden_no_adj(n, paths), vec![1, 2, 3, 4]);
}
