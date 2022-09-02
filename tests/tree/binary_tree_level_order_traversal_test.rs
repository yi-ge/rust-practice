use rust_practice::{
    libs::tree_node::vec_to_tree_node, tree, tree::binary_tree_level_order_traversal::Solution,
};

#[test]
fn level_order() {
    // 示例 1：
    // 输入：root = [3,9,20,null,null,15,7]
    // 输出：[[3],[9,20],[15,7]]
    assert_eq!(
        Solution::level_order(tree![3, 9, 20, None, None, 15, 7]),
        vec![vec![3], vec![9, 20], vec![15, 7]]
    );

    // 示例 2：
    // 输入：root = [1]
    // 输出：[[1]]
    assert_eq!(Solution::level_order(tree![1]), vec![vec![1]]);

    // 示例 3：
    // 输入：root = []
    // 输出：[]
    let vec: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::level_order(tree![]), vec);
}
