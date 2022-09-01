use rust_practice::array::sliding_window_maximum::Solution;

#[test]
fn max_sliding_window() {
    // 示例 1：
    // 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
    // 输出：[3,3,5,5,6,7]
    // 解释：
    // 滑动窗口的位置                最大值
    // ---------------               -----
    // [1  3  -1] -3  5  3  6  7       3
    //  1 [3  -1  -3] 5  3  6  7       3
    //  1  3 [-1  -3  5] 3  6  7       5
    //  1  3  -1 [-3  5  3] 6  7       5
    //  1  3  -1  -3 [5  3  6] 7       6
    //  1  3  -1  -3  5 [3  6  7]      7
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );

    // 示例 2：
    // 输入：nums = [1], k = 1
    // 输出：[1]
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
}
