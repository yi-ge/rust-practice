use std::collections::HashSet;

use rust_practice::tree::delete_nodes_and_return_forest::Solution;
use rust_practice::{libs::tree_node::tree_to_vec, libs::tree_node::vec_to_tree_node, tree};

// 注意，题目说忽略返回值顺序。因此需要考虑到这一点，否则会导致测试不通过。
// 在这个例子中，tree_to_vec函数使用了深度优先搜索来遍历树，然后返回一个包含所有节点值的向量。这个向量是经过排序的，所以你可以直接比较两个向量是否相等。
// 然后在测试函数中，我们首先将预期的结果转换为一个包含向量的HashSet，然后将Solution的输出转换为同样的格式，最后比较这两个HashSet。
#[test]
fn new() {
    // 示例 1：
    // 输入：root = [1,2,3,4,5,6,7], to_delete = [3,5]
    // 输出：[[1,2,null,4],[6],[7]]
    let root = tree![1, 2, 3, 4, 5, 6, 7];
    let to_delete = vec![3, 5];
    let res = Solution::del_nodes(root, to_delete);

    let mut expected: HashSet<_> = HashSet::new();
    expected.insert(vec![1, 2, 4]);
    expected.insert(vec![6]);
    expected.insert(vec![7]);

    let actual: HashSet<_> = res.into_iter().map(|root| tree_to_vec(root)).collect();

    assert_eq!(actual, expected);

    // 示例 2：
    // 输入：root = [1,2,4,null,3], to_delete = [3]
    // 输出：[[1,2,4]]
    let root = tree![1, 2, 4, null, 3];
    let to_delete = vec![3];
    let res = Solution::del_nodes(root, to_delete);

    let mut expected: HashSet<_> = HashSet::new();
    expected.insert(vec![1, 2, 4]);

    let actual: HashSet<_> = res.into_iter().map(|root| tree_to_vec(root)).collect();

    assert_eq!(actual, expected);
}
