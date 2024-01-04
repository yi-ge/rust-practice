use rust_practice::math::maximum_rows_covered_by_columns::Solution;

#[test]
fn maximum_rows() {
    // 示例 1：
    // 输入：matrix = [[0,0,0],[1,0,1],[0,1,1],[0,0,1]], numSelect = 2
    // 输出：3
    // 解释：
    // 图示中显示了一种覆盖 3 行的可行办法。
    // 选择 s = {0, 2} 。
    // - 第 0 行被覆盖，因为其中没有出现 1 。
    // - 第 1 行被覆盖，因为值为 1 的两列（即 0 和 2）均存在于 s 中。
    // - 第 2 行未被覆盖，因为 matrix[2][1] == 1 但是 1 未存在于 s 中。
    // - 第 3 行被覆盖，因为 matrix[2][2] == 1 且 2 存在于 s 中。
    // 因此，可以覆盖 3 行。
    // 另外 s = {1, 2} 也可以覆盖 3 行，但可以证明无法覆盖更多行。
    let matrix = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]];
    let num_select = 2;
    assert_eq!(Solution::maximum_rows(matrix, num_select), 3);

    // 示例 2：
    // 输入：matrix = [[1],[0]], numSelect = 1
    // 输出：2
    // 解释：
    // 选择唯一的一列，两行都被覆盖了，因为整个矩阵都被覆盖了。
    // 所以我们返回 2 。
    let matrix = vec![vec![1], vec![0]];
    let num_select = 1;
    assert_eq!(Solution::maximum_rows(matrix, num_select), 2);
}
