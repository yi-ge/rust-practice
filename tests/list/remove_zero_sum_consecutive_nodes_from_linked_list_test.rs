use rust_practice::list::remove_zero_sum_consecutive_nodes_from_linked_list::Solution;

use rust_practice::libs::list_node::list_node_to_vec;
use rust_practice::libs::list_node::vec_to_list_node;

#[test]
fn new() {
    // 示例 1：
    // 输入：head = [1,2,-3,3,1]
    // 输出：[3,1]
    // 提示：答案 [1,2,1] 也是正确的。
    let arr = vec![1, 2, -3, 3, 1];
    let head = vec_to_list_node(&arr);
    let res = Solution::remove_zero_sum_sublists(head);

    assert_eq!(list_node_to_vec(res), vec![3, 1]);

    // 示例 2：
    // 输入：head = [1,2,3,-3,4]
    // 输出：[1,2,4]
    let arr = vec![1, 2, 3, -3, 4];
    let head = vec_to_list_node(&arr);
    let res = Solution::remove_zero_sum_sublists(head);

    assert_eq!(list_node_to_vec(res), vec![1, 2, 4]);

    // 示例 3：
    // 输入：head = [1,2,3,-3,-2]
    // 输出：[1]
    let arr = vec![1, 2, 3, -3, -2];
    let head = vec_to_list_node(&arr);
    let res = Solution::remove_zero_sum_sublists(head);

    assert_eq!(list_node_to_vec(res), vec![1]);
}
