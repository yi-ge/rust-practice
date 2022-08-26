#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  pub fn new(val: i32) -> ListNode {
    ListNode {
      val,
      next: None
    }
  }
}
