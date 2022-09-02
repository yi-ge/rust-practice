use rust_practice::{
    libs::tree_node::vec_to_tree_node, stack::binary_tree_preorder_traversal::Solution,
};

#[test]
fn preorder_traversal() {
    // 示例 1：
    // 输入：root = [1,null,2,3]
    // 输出：[1,2,3]
    let root: Vec<Option<i32>> = vec![Some(1), None, Some(2), Some(3)];
    let ans = vec_to_tree_node(&root);
    // println!("{:?}", ans);
    let res_tree_node = Solution::preorder_traversal(ans);
    assert_eq!(res_tree_node, vec![1, 2, 3]);

    // 示例 2：
    // 输入：root = []
    // 输出：[]
    let root: Vec<Option<i32>> = vec![];
    let ans = vec_to_tree_node(&root);
    let res_tree_node = Solution::preorder_traversal(ans);
    assert_eq!(res_tree_node, vec![]);

    // 示例 3：
    // 输入：root = [1]
    // 输出：[1]
    let root: Vec<Option<i32>> = vec![Some(1)];
    let ans = vec_to_tree_node(&root);
    let res_tree_node = Solution::preorder_traversal(ans);
    assert_eq!(res_tree_node, vec![1]);

    // 示例 4：
    // 输入：root = [1,2]
    // 输出：[1,2]
    let root: Vec<Option<i32>> = vec![Some(1), Some(2)];
    let ans = vec_to_tree_node(&root);
    let res_tree_node = Solution::preorder_traversal(ans);
    assert_eq!(res_tree_node, vec![1, 2]);

    // 示例 5：
    // 输入：root = [1,null,2]
    // 输出：[1,2]
    let root: Vec<Option<i32>> = vec![Some(1), None, Some(2)];
    let ans = vec_to_tree_node(&root);
    let res_tree_node = Solution::preorder_traversal(ans);
    assert_eq!(res_tree_node, vec![1, 2]);
}
