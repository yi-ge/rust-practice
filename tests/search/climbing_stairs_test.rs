use rust_practice::search::climbing_stairs::Solution;

#[test]
fn climb_stairs() {
    // 示例 1：
    // 输入：n = 2
    // 输出：2
    // 解释：有两种方法可以爬到楼顶。
    // 1. 1 阶 + 1 阶
    // 2. 2 阶
    let n = 2;
    assert_eq!(Solution::climb_stairs(n), 2);

    // 示例 2：
    // 输入：n = 3
    // 输出：3
    // 解释：有三种方法可以爬到楼顶。
    // 1. 1 阶 + 1 阶 + 1 阶
    // 2. 1 阶 + 2 阶
    // 3. 2 阶 + 1 阶
    let n = 3;
    assert_eq!(Solution::climb_stairs(n), 3);
}
