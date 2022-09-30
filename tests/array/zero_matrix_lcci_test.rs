use rust_practice::array::zero_matrix_lcci::Solution;

#[test]
fn set_zeroes() {
    // 示例 1：
    // 输入：
    // [
    //   [1,1,1],
    //   [1,0,1],
    //   [1,1,1]
    // ]
    // 输出：
    // [
    //   [1,0,1],
    //   [0,0,0],
    //   [1,0,1]
    // ]
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

    // 示例 2：
    // 输入：
    // [
    //   [0,1,2,0],
    //   [3,4,5,2],
    //   [1,3,1,5]
    // ]
    // 输出：
    // [
    //   [0,0,0,0],
    //   [0,4,5,0],
    //   [0,3,1,0]
    // ]
    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
}
