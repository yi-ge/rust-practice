use rust_practice::{libs::list_node::ListNode, stack::remove_nodes_from_linked_list::Solution};

#[test]
fn new() {
    // 示例 1：
    // 输入：head = [5,2,13,3,8]
    // 输出：[13,8]
    // 解释：需要移除的节点是 5 ，2 和 3 。
    // - 节点 13 在节点 5 右侧。
    // - 节点 13 在节点 2 右侧。
    // - 节点 8 在节点 3 右侧。
    let mut head = Some(Box::new(ListNode::new(5)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(13)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(3)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(8)));
    let result = Solution::remove_nodes(head);

    // 更新断言以匹配预期的输出 [13, 8]
    // 检查第一个节点是否是13
    assert_eq!(result.as_ref().map(|node| node.val), Some(13));

    // 检查第二个节点是否是8
    assert_eq!(
        result
            .as_ref()
            .and_then(|node| node.next.as_ref().map(|node| node.val)),
        Some(8)
    );

    // 确保第二个节点之后没有更多的节点
    assert_eq!(
        result
            .as_ref()
            .and_then(|node| node.next.as_ref().and_then(|node| node.next.as_ref())),
        None
    );

    // 示例 2：
    // 输入：head = [1,1,1,1]
    // 输出：[1,1,1,1]
    // 解释：每个节点的值都是 1 ，所以没有需要移除的节点。
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(1)));
    let result = Solution::remove_nodes(head);
    // 更新断言以匹配预期的输出 [1, 1, 1, 1]
    // 检查第一个节点是否是1
    assert_eq!(result.as_ref().map(|node| node.val), Some(1));

    // 检查第二个节点是否是1
    assert_eq!(
        result
            .as_ref()
            .and_then(|node| node.next.as_ref().map(|node| node.val)),
        Some(1)
    );

    // 检查第三个节点是否是1
    assert_eq!(
        result.as_ref().and_then(|node| node
            .next
            .as_ref()
            .and_then(|node| node.next.as_ref().map(|node| node.val))),
        Some(1)
    );

    // 检查第四个节点是否是1
    assert_eq!(
        result
            .as_ref()
            .and_then(|node| node.next.as_ref().and_then(|node| node
                .next
                .as_ref()
                .and_then(|node| node.next.as_ref().map(|node| node.val)))),
        Some(1)
    );

    // 确保第四个节点之后没有更多的节点
    assert_eq!(
        result
            .as_ref()
            .and_then(|node| node.next.as_ref().and_then(|node| node
                .next
                .as_ref()
                .and_then(|node| node.next.as_ref().and_then(|node| node.next.as_ref())))),
        None
    );
}
