use rust_practice::math::moving_stones_until_consecutive::Solution;

#[test]
fn num_moves_stones() {
    // 示例 1：
    // 输入：a = 1, b = 2, c = 5
    // 输出：[1, 2]
    // 解释：将石子从 5 移动到 4 再移动到 3，或者我们可以直接将石子移动到 3。
    let a = 1;
    let b = 2;
    let c = 5;
    let result = Solution::num_moves_stones(a, b, c);
    assert_eq!(result, vec![1, 2]);

    // 示例 2：
    // 输入：a = 4, b = 3, c = 2
    // 输出：[0, 0]
    // 解释：我们无法进行任何移动。
    let a = 4;
    let b = 3;
    let c = 2;
    let result = Solution::num_moves_stones(a, b, c);
    assert_eq!(result, vec![0, 0]);
}
