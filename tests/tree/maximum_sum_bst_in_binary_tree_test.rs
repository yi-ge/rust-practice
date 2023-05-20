use rust_practice::tree::maximum_sum_bst_in_binary_tree::Solution;
use rust_practice::{libs::tree_node::vec_to_tree_node, tree};

#[test]
fn new() {
    // 示例 1：
    // 输入：root = [1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]
    // 输出：20
    // 解释：键值为 3 的子树是和最大的二叉搜索树。
    let root = tree![1, 4, 3, 2, 4, 2, 5, None, None, None, None, None, None, 4, 6];
    assert_eq!(Solution::max_sum_bst(root), 20);

    // 示例 2：
    // 输入：root = [4,3,null,1,2]
    // 输出：2
    // 解释：键值为 2 的单节点子树是和最大的二叉搜索树。
    let root = tree![4, 3, None, 1, 2];
    assert_eq!(Solution::max_sum_bst(root), 2);

    // 示例 3：
    // 输入：root = [-4,-2,-5]
    // 输出：0
    // 解释：所有节点键值都为负数，和最大的二叉搜索树为空。
    let root = tree![-4, -2, -5];
    assert_eq!(Solution::max_sum_bst(root), 0);

    // 示例 4：
    // 输入：root = [2,1,3]
    // 输出：6
    let root = tree![2, 1, 3];
    assert_eq!(Solution::max_sum_bst(root), 6);

    // 示例 5：
    // 输入：root = [5,4,8,3,null,6,3]
    // 输出：7
    let root = tree![5, 4, 8, 3, None, 6, 3];
    assert_eq!(Solution::max_sum_bst(root), 7);
}
