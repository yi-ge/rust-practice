use rust_practice::{
    libs::tree_node::vec_to_tree_node, tree, tree::binary_tree_preorder_traversal::Solution,
};

#[test]
fn preorder_traversal() {
    // 示例 1：
    // 输入：root = [1,null,2,3]
    // 输出：[1,2,3]
    assert_eq!(
        Solution::preorder_traversal(tree![1, None, 2, 3]),
        [1, 2, 3]
    );
    assert_eq!(
        Solution::preorder_traversal_recursive(tree![1, None, 2, 3]),
        [1, 2, 3]
    );

    // 示例 2：
    // 输入：root = []
    // 输出：[]
    assert_eq!(Solution::preorder_traversal(tree![]), []);
    assert_eq!(Solution::preorder_traversal_recursive(tree![]), []);

    // 示例 3：
    // 输入：root = [1]
    // 输出：[1]
    assert_eq!(Solution::preorder_traversal(tree![1]), [1]);
    assert_eq!(Solution::preorder_traversal_recursive(tree![1]), [1]);

    // 示例 4：
    // 输入：root = [1,2]
    // 输出：[1,2]
    assert_eq!(Solution::preorder_traversal(tree![1, 2]), [1, 2]);
    assert_eq!(Solution::preorder_traversal_recursive(tree![1, 2]), [1, 2]);

    // 示例 5：
    // 输入：root = [1,null,2]
    // 输出：[1,2]
    assert_eq!(Solution::preorder_traversal(tree![1, None, 2]), [1, 2]);
    assert_eq!(
        Solution::preorder_traversal_recursive(tree![1, None, 2]),
        [1, 2]
    );
}
