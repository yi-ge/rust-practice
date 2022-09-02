use rust_practice::{
    libs::tree_node::vec_to_tree_node, tree, tree::binary_tree_inorder_traversal::Solution,
};

#[test]
fn inorder_traversal() {
    // 示例 1：
    // 输入：root = [1,null,2,3]
    // 输出：[1,3,2]
    assert_eq!(
        Solution::inorder_traversal(tree![1, None, 2, 3]),
        vec![1, 3, 2]
    );
    assert_eq!(
        Solution::inorder_traversal_recursive(tree![1, None, 2, 3]),
        vec![1, 3, 2]
    );

    // 示例 2：
    // 输入：root = []
    // 输出：[]
    assert_eq!(Solution::inorder_traversal(tree![]), vec![]);
    assert_eq!(Solution::inorder_traversal_recursive(tree![]), vec![]);

    // 示例 3：
    // 输入：root = [1]
    // 输出：[1]
    assert_eq!(Solution::inorder_traversal(tree![1]), vec![1]);
    assert_eq!(Solution::inorder_traversal_recursive(tree![1]), vec![1]);
}
