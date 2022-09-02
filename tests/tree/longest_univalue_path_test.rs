use rust_practice::tree::longest_univalue_path::Solution;
use rust_practice::{libs::tree_node::vec_to_tree_node, tree};

#[test]
fn longest_univalue_path() {
    // 示例 1:
    // 输入：root = [5,4,5,1,1,5]
    // 输出：2
    assert_eq!(Solution::longest_univalue_path(tree![5, 4, 5, 1, 1, 5]), 2);

    // 示例 2:
    // 输入：root = [1,4,5,4,4,5]
    // 输出：2
    assert_eq!(Solution::longest_univalue_path(tree![1, 4, 5, 4, 4, 5]), 2);
}
