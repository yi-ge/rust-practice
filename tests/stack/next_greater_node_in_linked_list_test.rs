use rust_practice::{libs::list_node::ListNode, stack::next_greater_node_in_linked_list::Solution};

#[test]
fn new() {
    // 示例 1：
    // 输入：head = [2,1,5]
    // 输出：[5,5,0]
    let mut head1 = Box::new(ListNode::new(2));
    head1.next = Some(Box::new(ListNode::new(1)));
    head1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

    let result1 = Solution::next_larger_nodes(Some(head1));
    assert_eq!(result1, vec![5, 5, 0]);

    // 示例 2：
    // 输入：head = [2,7,4,3,5]
    // 输出：[7,0,5,5,0]
    let mut head2 = Box::new(ListNode::new(2));
    head2.next = Some(Box::new(ListNode::new(7)));
    head2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    head2.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head2
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(5)));

    let result2 = Solution::next_larger_nodes(Some(head2));
    assert_eq!(result2, vec![7, 0, 5, 5, 0]);
}
