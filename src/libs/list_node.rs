#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn vec_to_list_node(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in 0..vec.len() {
        let node = Some(Box::new(ListNode {
            val: vec[i],
            next: None,
        }));
        match head.as_ref() {
            None => head = node,
            Some(_) => {
                let head_mut = head.as_mut().unwrap();
                let mut head_res = &mut *(head_mut);
                while head_res.next.is_some() {
                    head_res = &mut *(head_res.next.as_mut().unwrap())
                }
                head_res.next = node;
            }
        }
    }
    head
}

pub fn list_node_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut ref_node = head.as_ref();
    while ref_node.is_some() {
        res.push(ref_node.unwrap().val);
        ref_node = ref_node.unwrap().next.as_ref();
    }
    res
}
