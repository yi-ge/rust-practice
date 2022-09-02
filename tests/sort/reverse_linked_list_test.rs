use rust_practice::libs::list_node::{list_node_to_vec, vec_to_list_node};
use rust_practice::sort::reverse_linked_list::Solution;

#[test]
fn new() {
    // 示例 1：
    // 输入：head = [1,2,3,4,5]
    // 输出：[5,4,3,2,1]
    let head = vec![1, 2, 3, 4, 5];
    let list = Solution::reverse_list(vec_to_list_node(&head));
    let out = list_node_to_vec(list);
    assert_eq!(&out[..], &[5, 4, 3, 2, 1][..]);

    // 示例 2：
    // 输入：head = [1,2]
    // 输出：[2,1]
    let head = vec![1, 2];
    let list = Solution::reverse_list(vec_to_list_node(&head));
    let out = list_node_to_vec(list);
    assert_eq!(&out[..], &[2, 1][..]);

    // 示例 3：
    // 输入：head = []
    // 输出：[]
    let head = vec![];
    let list = Solution::reverse_list(vec_to_list_node(&head));
    let out = list_node_to_vec(list);
    assert_eq!(&out[..], &[][..]);
}
