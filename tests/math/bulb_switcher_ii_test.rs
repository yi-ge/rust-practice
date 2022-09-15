use rust_practice::math::bulb_switcher_ii::Solution;

#[test]
fn flip_lights() {
    // 示例 1：
    // 输入：n = 1, presses = 1
    // 输出：2
    // 解释：状态可以是：
    // - 按压开关 1 ，[关]
    // - 按压开关 2 ，[开]
    assert_eq!(Solution::flip_lights(1, 1), 2);

    // 示例 2：
    // 输入：n = 2, presses = 1
    // 输出：3
    // 解释：状态可以是：
    // - 按压开关 1 ，[关, 关]
    // - 按压开关 2 ，[开, 关]
    // - 按压开关 3 ，[关, 开]
    assert_eq!(Solution::flip_lights(2, 1), 3);

    // 示例 3：
    // 输入：n = 3, presses = 1
    // 输出：4
    // 解释：状态可以是：
    // - 按压开关 1 ，[关, 关, 关]
    // - 按压开关 2 ，[关, 开, 关]
    // - 按压开关 3 ，[开, 关, 开]
    // - 按压开关 4 ，[关, 开, 开]
    assert_eq!(Solution::flip_lights(3, 1), 4);

    assert_eq!(Solution::flip_lights(1, 0), 1);
    assert_eq!(Solution::flip_lights(8, 2), 7);
    assert_eq!(Solution::flip_lights(8, 3), 8);
}
