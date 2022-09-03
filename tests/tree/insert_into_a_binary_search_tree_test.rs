use rust_practice::{
    libs::tree_node::vec_to_tree_node, tree, tree::insert_into_a_binary_search_tree::Solution,
};

#[test]
fn insert_into_bst() {
    // 示例 1：
    // 输入：root = [4,2,7,1,3], val = 5
    // 输出：[4,2,7,1,3,5]
    // 解释：另一个满足题目要求可以通过的树是 [5, 2, 7, 1, 3, 4]
    assert_eq!(
        Solution::insert_into_bst(tree![4, 2, 7, 1, 3], 5),
        tree![4, 2, 7, 1, 3, 5]
    );

    // 示例 2：
    // 输入：root = [40,20,60,10,30,50,70], val = 25
    // 输出：[40,20,60,10,30,50,70,null,null,25]
    assert_eq!(
        Solution::insert_into_bst(tree![40, 20, 60, 10, 30, 50, 70], 25),
        tree![40, 20, 60, 10, 30, 50, 70, None, None, 25]
    );

    // 示例 3：
    // 输入：root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
    // 输出：[4,2,7,1,3,5]
    assert_eq!(
        Solution::insert_into_bst(tree![4, 2, 7, 1, 3, None, None, None, None, None, None], 5),
        tree![4, 2, 7, 1, 3, 5]
    );

    // 示例 4：
    // 输入：root = [], val = 1
    // 输出：[1]
    assert_eq!(Solution::insert_into_bst(tree![], 1), tree![1]);
}
