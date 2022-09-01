use rust_practice::{
    libs::list_node::{list_node_to_vec, vec_to_list_node},
    list::remove_nth_node_from_end_of_list::Solution,
};

#[test]
fn remove_nth_from_end() {
    // 示例 1：
    // 输入：head = [1,2,3,4,5], n = 2
    // 输出：[1,2,3,5]
    let head = vec![1, 2, 3, 4, 5];
    let res = Solution::remove_nth_from_end(vec_to_list_node(head), 2);
    assert_eq!(list_node_to_vec(res), vec![1, 2, 3, 5]);

    // 示例 2：
    // 输入：head = [1], n = 1
    // 输出：[]
    let head = vec![1];
    let res = Solution::remove_nth_from_end(vec_to_list_node(head), 1);
    assert_eq!(list_node_to_vec(res), vec![]);

    // 示例 3：
    // 输入：head = [1,2], n = 1
    // 输出：[1]
    let head = vec![1, 2];
    let res = Solution::remove_nth_from_end(vec_to_list_node(head), 1);
    assert_eq!(list_node_to_vec(res), vec![1]);
}
