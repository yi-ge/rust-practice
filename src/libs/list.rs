use super::list_node::ListNode;

#[derive(Clone, Debug)]
pub struct List {
    pub head: Option<Box<ListNode>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    /// 在链表末尾插入节点
    pub fn push_back(&mut self, val: i32) {
        let node = Some(Box::new(ListNode { val, next: None }));
        match self.head.as_ref() {
            None => self.head = node,
            Some(_) => {
                let head_mut = self.head.as_mut().unwrap();
                let mut head_res = &mut *(head_mut);
                while head_res.next.is_some() {
                    head_res = &mut *(head_res.next.as_mut().unwrap())
                }
                head_res.next = node;
            }
        }
    }

    /// 在链表头部插入节点
    pub fn push_front(&mut self, val: i32) {
        self.head = Some(Box::new(ListNode {
            val,
            next: self.head.take(),
        }));
    }

    /// 移除链表head节点
    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                let val = node.val;
                self.head = node.next;
                return Some(val);
            }
        }
    }

    /// 返回链表head节点的val值
    pub fn peek(&mut self) -> Option<i32> {
        let node = self.head.as_ref();
        match node {
            None => None,
            Some(node) => Some(node.val),
        }
    }

    /// 反转链表
    pub fn reverse(&mut self) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = self.head.take() {
            self.head = node.next;
            node.next = prev;
            prev = Some(node);
        }
        self.head = prev;
        self.head.clone()
    }

    /// 插入链表，在指定位置处插入节点
    pub fn insert(&mut self, mut node: Option<Box<ListNode>>, index: i32) -> bool {
        match self.head.as_ref() {
            None => {
              if index == 0 {
                self.head = node;
                return true
              }
            },
            Some(_) => {
                let head_mut = self.head.as_mut().unwrap();
                let mut head_res = &mut *(head_mut);

                if index == 0 {
                  let head_node = self.head.take();
                  node.as_mut().unwrap().next = head_node;
                  self.head = node;
                  return true;
                }

                let mut i = 0;
                while head_res.next.is_some() && i != index - 1 {
                    head_res = &mut *(head_res.next.as_mut().unwrap());
                    i += 1;
                }

                if head_res.next.is_some() && i == index - 1 {
                  let next_node = head_res.next.take().unwrap();
                  node.as_mut().unwrap().next = Some(next_node);
                  head_res.next = node;
                  return true
                } else if !head_res.next.is_some() && i == index - 1 {
                  head_res.next = node;
                  return true
                }
            }
        }

        return false
    }

    /// 打印链表
    pub fn print_list(&mut self) -> String {
        let mut out = String::new();

        let mut ref_node = self.head.as_ref();
        while ref_node.is_some() {
            out.push_str(&ref_node.unwrap().val.to_string());
            out.push_str(" -> ");
            ref_node = ref_node.unwrap().next.as_ref();
        }

        out.push_str("None");
        println!("{}", out);
        out
    }
}
