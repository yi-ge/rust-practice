use rust_practice::tree::maximum_difference_between_node_and_ancestor::Solution;
use rust_practice::{libs::tree_node::vec_to_tree_node, tree};

#[test]
fn new() {
    // 示例 1：
    // 输入：root = [8,3,10,1,6,null,14,null,null,4,7,13]
    // 输出：7
    // 解释：
    // 我们有大量的节点与其祖先的差值，其中一些如下：
    // |8 - 3| = 5
    // |3 - 7| = 4
    // |8 - 1| = 7
    // |10 - 13| = 3
    // 在所有可能的差值中，最大值 7 由 |8 - 1| = 7 得出。
    assert_eq!(
        Solution::max_ancestor_diff(tree![8, 3, 10, 1, 6, None, 14, None, None, 4, 7, 13]),
        7
    );

    // 示例 2：
    // 输入：root = [1,null,2,null,0,3]
    // 输出：3
    assert_eq!(
        Solution::max_ancestor_diff(tree![1, None, 2, None, 0, 3]),
        3
    );

    // 测试：
    // 输入：root = [0]
    // 输出：0
    assert_eq!(Solution::max_ancestor_diff(tree![0]), 0);
}
