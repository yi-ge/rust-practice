use rust_practice::libs::list::List;

#[test]
fn new() {
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
}
