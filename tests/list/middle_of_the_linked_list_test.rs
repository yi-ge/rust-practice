use std::vec;

use rust_practice::{libs::list_node::vec_to_list_node, list::middle_of_the_linked_list::Solution};

#[test]
fn middle_node() {
    // 示例 1：
    // 输入：[1,2,3,4,5]
    // 输出：此列表中的结点 3 (序列化形式：[3,4,5])
    // 返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
    // 注意，我们返回了一个 ListNode 类型的对象 ans，这样：
    // ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next = NULL.
    let list = vec![1, 2, 3, 4, 5];
    let node = Solution::middle_node(vec_to_list_node(list));
    assert_eq!(node.as_ref().unwrap().val, 3);

    // 示例 2：
    // 输入：[1,2,3,4,5,6]
    // 输出：此列表中的结点 4 (序列化形式：[4,5,6])
    // 由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。
    let list = vec![1, 2, 3, 4, 5, 6];
    let node = Solution::middle_node(vec_to_list_node(list));
    assert_eq!(node.as_ref().unwrap().val, 4);
}
