use rust_practice::libs::list_node::ListNode;

#[test]
fn new() {
    let mut node = Some(Box::new(ListNode { val: 1, next: None }));

    assert_eq!(node.as_mut().unwrap().val, 1);
    assert_eq!(node.as_ref().unwrap().next == None, true);
}
