use rust_practice::{
    libs::tree_node::vec_to_tree_node, tree, tree::binary_tree_postorder_traversal::Solution,
};

#[test]
fn postorder_traversal() {
    // 示例 1：
    // 输入：root = [1,null,2,3]
    // 输出：[3,2,1]
    assert_eq!(
        Solution::postorder_traversal(tree![1, None, 2, 3]),
        vec![3, 2, 1]
    );
    assert_eq!(
        Solution::postorder_traversal_recursive(tree![1, None, 2, 3]),
        vec![3, 2, 1]
    );

    // 示例 2：
    // 输入：root = []
    // 输出：[]
    assert_eq!(Solution::postorder_traversal(tree![]), vec![]);
    assert_eq!(Solution::postorder_traversal_recursive(tree![]), vec![]);

    // 示例 3：
    // 输入：root = [1]
    // 输出：[1]
    assert_eq!(Solution::postorder_traversal(tree![1]), vec![1]);
    assert_eq!(Solution::postorder_traversal_recursive(tree![1]), vec![1]);
}
