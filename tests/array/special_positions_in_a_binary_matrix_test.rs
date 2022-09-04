use rust_practice::array::special_positions_in_a_binary_matrix::Solution;

#[test]
fn num_special() {
    // 示例 1：
    // 输入：mat = [[1,0,0],
    //             [0,0,1],
    //             [1,0,0]]
    // 输出：1
    // 解释：(1,2) 是一个特殊位置，因为 mat[1][2] == 1 且所处的行和列上所有其他元素都是 0
    assert_eq!(
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
        1
    );

    // 示例 2：
    // 输入：mat = [[1,0,0],
    //             [0,1,0],
    //             [0,0,1]]
    // 输出：3
    // 解释：(0,0), (1,1) 和 (2,2) 都是特殊位置
    assert_eq!(
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );

    // 示例 3：
    // 输入：mat = [[0,0,0,1],
    //             [1,0,0,0],
    //             [0,1,1,0],
    //             [0,0,0,0]]
    // 输出：2
    assert_eq!(
        Solution::num_special(vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        2
    );

    // 示例 4：
    // 输入：mat = [[0,0,0,0,0],
    //             [1,0,0,0,0],
    //             [0,1,0,0,0],
    //             [0,0,1,0,0],
    //             [0,0,0,1,1]]
    // 输出：3
    assert_eq!(
        Solution::num_special(vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1]
        ]),
        3
    );
}
