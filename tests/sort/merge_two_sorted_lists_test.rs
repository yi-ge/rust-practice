use rust_practice::{
    libs::list_node::list_node_to_vec, libs::list_node::vec_to_list_node,
    sort::merge_two_sorted_lists::Solution,
};

#[test]
fn merge_two_lists() {
    // 示例 1：
    // 输入：l1 = [1,2,4], l2 = [1,3,4]
    // 输出：[1,1,2,3,4,4]
    let l1 = vec![1, 2, 4];
    let l2 = vec![1, 3, 4];
    assert_eq!(
        list_node_to_vec(Solution::merge_two_lists(
            vec_to_list_node(&l1),
            vec_to_list_node(&l2)
        )),
        &vec![1, 1, 2, 3, 4, 4][..]
    );

    // 示例 2：
    // 输入：l1 = [], l2 = []
    // 输出：[]
    let l1 = vec![];
    let l2 = vec![];
    assert_eq!(
        list_node_to_vec(Solution::merge_two_lists(
            vec_to_list_node(&l1),
            vec_to_list_node(&l2)
        )),
        &vec![][..]
    );

    // 示例 3：
    // 输入：l1 = [], l2 = [0]
    // 输出：[0]
    let l1 = vec![];
    let l2 = vec![0];
    assert_eq!(
        list_node_to_vec(Solution::merge_two_lists(
            vec_to_list_node(&l1),
            vec_to_list_node(&l2)
        )),
        &vec![0][..]
    );
}
