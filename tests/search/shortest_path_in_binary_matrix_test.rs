use rust_practice::search::shortest_path_in_binary_matrix::Solution;

#[test]
fn shortest_path_binary_matrix() {
    // 示例 1：
    // 输入：grid = [[0,1],[1,0]]
    // 输出：2
    let grid = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);

    // 示例 2：
    // 输入：grid = [[0,0,0],[1,1,0],[1,1,0]]
    // 输出：4
    let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);

    // 示例 3：
    // 输入：grid = [[1,0,0],[1,1,0],[1,1,0]]
    // 输出：-1
    let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
}
