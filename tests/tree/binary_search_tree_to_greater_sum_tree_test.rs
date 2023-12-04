use rust_practice::{
    libs::tree_node::TreeNode, tree::binary_search_tree_to_greater_sum_tree::Solution,
};

#[test]
fn test_tree_node() {
    // 示例 1：
    // 输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
    // 输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
    // 构建输入的二叉树
    let tree1 = TreeNode::from_vec(vec![
        Some(4),
        Some(1),
        Some(6),
        Some(0),
        Some(2),
        Some(5),
        Some(7),
        None,
        None,
        None,
        Some(3),
        None,
        None,
        None,
        Some(8),
    ]);
    // 调用函数
    let result1 = Solution::bst_to_gst(tree1);
    // 构建预期输出
    let expected1 = TreeNode::from_vec(vec![
        Some(30),
        Some(36),
        Some(21),
        Some(36),
        Some(35),
        Some(26),
        Some(15),
        None,
        None,
        None,
        Some(33),
        None,
        None,
        None,
        Some(8),
    ]);
    // 断言
    assert_eq!(result1, expected1);

    // 示例 2：
    // 输入：root = [0,null,1]
    // 输出：[1,null,1]
    // 构建输入的二叉树
    let tree2 = TreeNode::from_vec(vec![Some(0), None, Some(1)]);
    // 调用函数
    let result2 = Solution::bst_to_gst(tree2);
    // 构建预期输出
    let expected2 = TreeNode::from_vec(vec![Some(1), None, Some(1)]);
    // 断言
    assert_eq!(result2, expected2);
}
