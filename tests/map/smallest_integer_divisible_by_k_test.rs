use rust_practice::map::smallest_integer_divisible_by_k::Solution;

#[test]
fn smallest_repunit_div_by_k() {
    // 示例 1：
    // 输入：k = 1
    // 输出：1
    // 解释：最小的答案是 n = 1，其长度为 1。
    assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);

    // 示例 2：
    // 输入：k = 2
    // 输出：-1
    // 解释：不存在可被 2 整除的正整数 n 。
    assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);

    // 示例 3：
    // 输入：k = 3
    // 输出：3
    // 解释：最小的答案是 n = 111，其长度为 3。
    assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
}
