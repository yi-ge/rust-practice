use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// fn create_tree(vec: &Vec<Option<i32>>, index: usize) -> Option<Rc<RefCell<TreeNode>>> {
//     if index < vec.len() {
//         // && !vec.get(index).is_none()
//         if let Some(v) = vec[index] {
//             let root = Rc::new(RefCell::new(TreeNode::new(v)));

//             root.borrow_mut().left = create_tree(&vec, index * 2 + 1);
//             root.borrow_mut().right = create_tree(&vec, index * 2 + 2);

//             return Some(root);
//         }
//     }

//     None
// }

// pub fn vec_to_tree_node2(vec: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
//     create_tree(&vec, 0)
// }

pub fn vec_to_tree_node(vec: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.get(0).is_none() {
        return None;
    }
    if let Some(v) = vec[0] {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());
        for children in vec[1..].chunks(2) {
            let mut iter = children.into_iter();
            let parent = queue.pop_front().unwrap();
            if let Some(Some(v)) = iter.next() {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if let Some(Some(v)) = iter.next() {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
        root
    } else {
        None
    }
}
