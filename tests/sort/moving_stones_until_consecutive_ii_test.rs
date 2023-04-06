use rust_practice::sort::moving_stones_until_consecutive_ii::Solution;

#[test]
fn num_moves_stones_ii() {
    // 示例 1：
    // 输入：[7,4,9]
    // 输出：[1,2]
    // 解释：
    // 我们可以移动一次，4 -> 8，游戏结束。
    // 或者，我们可以移动两次 9 -> 5，4 -> 6，游戏结束。
    let stones1 = vec![7, 4, 9];
    let expected1 = vec![1, 2];
    assert_eq!(Solution::num_moves_stones_ii(stones1), expected1);

    // 示例 2：
    // 输入：[6,5,4,3,10]
    // 输出：[2,3]
    // 解释：
    // 我们可以移动 3 -> 8，接着是 10 -> 7，游戏结束。
    // 或者，我们可以移动 3 -> 7, 4 -> 8, 5 -> 9，游戏结束。
    // 注意，我们无法进行 10 -> 2 这样的移动来结束游戏，因为这是不合要求的移动。
    let stones2 = vec![6, 5, 4, 3, 10];
    let expected2 = vec![2, 3];
    assert_eq!(Solution::num_moves_stones_ii(stones2), expected2);

    // 示例 3：
    // 输入：[100,101,104,102,103]
    // 输出：[0,0]
    let stones3 = vec![100, 101, 104, 102, 103];
    let expected3 = vec![0, 0];
    assert_eq!(Solution::num_moves_stones_ii(stones3), expected3);
}
