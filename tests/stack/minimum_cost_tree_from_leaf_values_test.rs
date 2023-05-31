use rust_practice::stack::minimum_cost_tree_from_leaf_values::Solution;

#[test]
fn mct_from_leaf_values() {
    // 示例 1：
    // 输入：arr = [6,2,4]
    // 输出：32
    // 解释：有两种可能的树，第一种的非叶节点的总和为 36 ，第二种非叶节点的总和为 32 。
    let arr = vec![6, 2, 4];
    assert_eq!(Solution::mct_from_leaf_values(arr), 32);

    // 示例 2：
    // 输入：arr = [4,11]
    // 输出：44
    let arr = vec![4, 11];
    assert_eq!(Solution::mct_from_leaf_values(arr), 44);
}
