use rust_practice::{
    libs::list_node::{list_node_to_vec, vec_to_list_node},
    list::remove_duplicates_from_sorted_list_ii::Solution,
};

#[test]
fn remove_duplicates_from_sorted_list_ii_test() {
    // 示例 1：
    // 输入：head = [1,2,3,3,4,4,5]
    // 输出：[1,2,5]
    let head = vec![1, 2, 3, 3, 4, 4, 5];
    let res = Solution::delete_duplicates(vec_to_list_node(&head));
    assert_eq!(list_node_to_vec(res), vec![1, 2, 5]);

    // 示例 2：
    // 输入：head = [1,1,1,2,3]
    // 输出：[2,3]
    let head = vec![1, 1, 1, 2, 3];
    let res = Solution::delete_duplicates(vec_to_list_node(&head));
    assert_eq!(list_node_to_vec(res), vec![2, 3]);
}
