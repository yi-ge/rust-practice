use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
        let mut queue = VecDeque::new();
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

pub fn tree_node_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut res: Vec<Option<i32>> = vec![];
    if root.is_none() {
        return res;
    }

    let mut deque = VecDeque::new();
    deque.push_back(root);

    while !deque.is_empty() {
        for _ in 0..deque.len() {
            let node = deque.pop_front();
            if let Some(Some(v)) = node {
                // 当前节点
                res.push(Some(v.borrow().val));

                // 左节点
                if v.borrow().left.is_some() {
                    deque.push_back(v.borrow().left.clone());
                } else {
                    deque.push_back(None);
                }

                // 右节点
                if v.borrow().right.is_some() {
                    deque.push_back(v.borrow().right.clone());
                } else {
                    deque.push_back(None);
                }
            } else {
                res.push(None);
            }
        }
    }

    for i in (0..res.len()).rev() {
        if res[i].is_none() {
            res.pop();
        } else {
            break;
        }
    }

    res
}

// 用于比较的辅助方法
// tree_to_vec 函数使用了深度优先搜索来遍历树，然后返回一个包含所有节点值的向量。这个向量是经过排序的，所以你可以直接比较两个向量是否相等。
pub fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = Vec::new();
    if let Some(node) = root {
        stack.push(node.clone());
    }
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        let node = node.borrow();
        res.push(node.val);
        if let Some(right) = &node.right {
            stack.push(right.clone());
        }
        if let Some(left) = &node.left {
            stack.push(left.clone());
        }
    }
    res.sort_unstable();
    res
}

#[macro_export]
macro_rules! tree {
    () => { None };
    ($( $e:expr ), *) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec: Vec<_> = vec
                .into_iter()
                .map(|x| x.parse::<i32>().ok()).collect();
            vec_to_tree_node(&vec)
        }
    };
    ($( $e:expr, ) *) => { tree![$($e), *] };
}
