use rust_practice::other::tiling_a_rectangle_with_the_fewest_squares::Solution;

#[test]
fn tiling_rectangle() {
    // 示例 1：
    // 输入：n = 2, m = 3
    // 输出：3
    // 解释：3 块地砖就可以铺满卧室。
    //      2 块 1x1 地砖
    //      1 块 2x2 地砖
    let n = 2;
    let m = 3;
    assert_eq!(Solution::tiling_rectangle(n, m), 3);

    // 示例 2：
    // 输入：n = 5, m = 8
    // 输出：5
    let n = 5;
    let m = 8;
    assert_eq!(Solution::tiling_rectangle(n, m), 5);

    // 示例 3：
    // 输入：n = 11, m = 13
    // 输出：6
    let n = 11;
    let m = 13;
    assert_eq!(Solution::tiling_rectangle(n, m), 6);
}
