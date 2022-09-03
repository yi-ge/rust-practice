use rust_practice::libs::list_node::{list_node_to_vec, vec_to_list_node, ListNode};

#[test]
fn list_node_new() {
    let mut node = Some(Box::new(ListNode::new(1)));

    assert_eq!(node.as_mut().unwrap().val, 1);
    assert_eq!(node.as_ref().unwrap().next == None, true);

    assert_eq!(ListNode::new(1), ListNode { val: 1, next: None });
    assert_eq!(ListNode::new(2).clone().val, 2);
}

#[test]
fn test_vec_to_list_node() {
    let vec = vec![1, 2, 3];
    let list = vec_to_list_node(&vec);
    assert_eq!(list.as_ref().unwrap().val, 1);
    assert_eq!(list.as_ref().unwrap().next.as_ref().unwrap().val, 2);
    assert_eq!(
        list.as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .val,
        3
    );

    assert_eq!(vec_to_list_node(&vec![]), None);
}

#[test]
fn test_list_node_to_vec() {
    let vec = vec![1, 2, 3];
    let list = vec_to_list_node(&vec);
    let out = list_node_to_vec(list);
    assert_eq!(out.len(), 3);
    assert_eq!(out[0], 1);
    assert_eq!(out[1], 2);
    assert_eq!(out[2], 3);

    assert_eq!(list_node_to_vec(vec_to_list_node(&vec![])), vec![]);
}
