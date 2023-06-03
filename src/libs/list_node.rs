#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // 创建一个新节点
    // * #[inline] 如果存在inline则单元覆盖测试不到此函数，对性能要求不高，故注释
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// 将一个整数数组转换为链表
pub fn vec_to_list_node(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in 0..vec.len() {
        let node = Some(Box::new(ListNode {
            val: vec[i],
            next: None,
        }));
        // 如果头节点为空，则将当前节点设置为头节点
        // 如果头节点不为空，则将当前节点添加到链表尾部
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

// 将链表转换为整数数组
pub fn list_node_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut ref_node = head.as_ref();
    // 遍历链表，将每个节点的值加入到数组中
    while ref_node.is_some() {
        res.push(ref_node.unwrap().val);
        ref_node = ref_node.unwrap().next.as_ref();
    }
    res
}
