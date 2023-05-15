use rust_practice::array::flip_columns_for_maximum_number_of_equal_rows::Solution;

#[test]
fn max_equal_rows_after_flips() {
    // 示例 1：
    // 输入：matrix = [[0,1],[1,1]]
    // 输出：1
    // 解释：不进行翻转，有 1 行所有值都相等。
    let matrix = vec![vec![0, 1], vec![1, 1]];
    assert_eq!(Solution::max_equal_rows_after_flips(matrix), 1);

    // 示例 2：
    // 输入：matrix = [[0,1],[1,0]]
    // 输出：2
    // 解释：翻转第一列的值之后，这两行都由相等的值组成。
    let matrix = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::max_equal_rows_after_flips(matrix), 2);

    // 示例 3：
    // 输入：matrix = [[0,0,0],[0,0,1],[1,1,0]]
    // 输出：2
    // 解释：翻转前两列的值之后，后两行由相等的值组成。
    let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
    assert_eq!(Solution::max_equal_rows_after_flips(matrix), 2);
}
