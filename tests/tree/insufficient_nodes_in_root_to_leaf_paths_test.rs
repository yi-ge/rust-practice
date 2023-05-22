use rust_practice::tree::insufficient_nodes_in_root_to_leaf_paths::Solution;
use rust_practice::{libs::tree_node::vec_to_tree_node, tree};

#[test]
fn new() {
    // 示例 1：
    // 输入：root = [1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14], limit = 1
    // 输出：[1,2,3,4,null,null,7,8,9,null,14]
    let root = tree![1, 2, 3, 4, -99, -99, 7, 8, 9, -99, -99, 12, 13, -99, 14];
    let limit = 1;
    let res = tree![1, 2, 3, 4, None, None, 7, 8, 9, None, 14];
    assert_eq!(Solution::sufficient_subset(root, limit), res);

    // 示例 2：
    // 输入：root = [5,4,8,11,null,17,4,7,1,null,null,5,3], limit = 22
    // 输出：[5,4,8,11,null,17,4,7,null,null,null,5]
    let root = tree![5, 4, 8, 11, null, 17, 4, 7, 1, null, null, 5, 3];
    let limit = 22;
    let res = tree![5, 4, 8, 11, null, 17, 4, 7, null, null, null, 5];
    assert_eq!(Solution::sufficient_subset(root, limit), res);

    // 示例 3：
    // 输入：root = [1,2,-3,-5,null,4,null], limit = -1
    // 输出：[1,null,-3,4]
    let root = tree![1, 2, -3, -5, null, 4, null];
    let limit = -1;
    let res = tree![1, null, -3, 4];
    assert_eq!(Solution::sufficient_subset(root, limit), res);
}
