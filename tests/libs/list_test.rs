use rust_practice::libs::{list::List, list_node::ListNode};

#[test]
fn list_new() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.print_list();
    assert_eq!(list.peek(), Some(3));

    assert_eq!(list.pop(), Some(3));
    list.print_list();
    assert_eq!(list.peek(), Some(2));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.peek(), None);

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.peek(), Some(1));
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));

    assert_eq!(List::new(), List { head: None });

    let list_struct = List { head: None };
    println!("{:?}", list_struct);
    assert_eq!(list_struct.head, None);
    assert_eq!(list_struct.clone().head, None);
    assert_eq!(list_struct, list_struct.clone());
}

#[test]
fn reverse() {
    let mut list = List::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    list.reverse();
    list.print_list();
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
}

#[test]
fn inset() {
    // 1 -> 4 -> 2 -> 3 -> None
    let mut list = List::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = Some(Box::new(ListNode::new(4)));
    assert_eq!(list.insert(node, 1), true);
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));

    // 1 -> 2 -> 4 -> 3 -> None
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = Some(Box::new(ListNode::new(4)));
    assert_eq!(list.insert(node, 2), true);
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(3));

    // 1 -> 2 -> 3 -> 4 -> None
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = Some(Box::new(ListNode::new(4)));
    assert_eq!(list.insert(node, 3), true);
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));

    // 4 -> 1 -> 2 -> 3 -> None
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = Some(Box::new(ListNode::new(4)));
    assert_eq!(list.insert(node, 0), true);
    list.print_list();
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));

    // -1
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = Some(Box::new(ListNode::new(4)));
    assert_eq!(list.insert(node, -1), false);
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));

    // 4
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = Some(Box::new(ListNode::new(4)));
    assert_eq!(list.insert(node, 4), false);
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));
}

#[test]
fn test_get_middle_node() {
    let mut list = List::new();
    list.push_front(5);
    list.push_front(4);
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print_list();
    let node = list.get_middle_node();
    assert_eq!(node.as_ref().unwrap().val, 3);
    list.print_list();
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));
    assert_eq!(list.pop(), Some(5));
}
