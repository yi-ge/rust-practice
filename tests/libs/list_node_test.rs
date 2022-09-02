use rust_practice::libs::list_node::{list_node_to_vec, vec_to_list_node, ListNode};

#[test]
fn new() {
    let mut node = Some(Box::new(ListNode { val: 1, next: None }));

    assert_eq!(node.as_mut().unwrap().val, 1);
    assert_eq!(node.as_ref().unwrap().next == None, true);
}

#[test]
fn vec_to_list_node_test() {
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
}

#[test]
fn list_node_to_vec_test() {
    let vec = vec![1, 2, 3];
    let list = vec_to_list_node(&vec);
    let out = list_node_to_vec(list);
    assert_eq!(out.len(), 3);
    assert_eq!(out[0], 1);
    assert_eq!(out[1], 2);
    assert_eq!(out[2], 3);
}
