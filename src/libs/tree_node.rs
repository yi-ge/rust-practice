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

    pub fn from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if values.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while i < values.len() {
            let parent = queue.pop_front().unwrap();

            if i < values.len() && values[i].is_some() {
                let left_child = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                parent.borrow_mut().left = Some(left_child.clone());
                queue.push_back(left_child);
            }
            i += 1;

            if i < values.len() && values[i].is_some() {
                let right_child = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                parent.borrow_mut().right = Some(right_child.clone());
                queue.push_back(right_child);
            }
            i += 1;
        }

        Some(root)
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
        // 如果向量为空，则返回 None
        return None;
    }
    if let Some(v) = vec[0] {
        // 否则，新建一个根节点
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        let mut queue = VecDeque::new(); // 创建一个队列，用于层次遍历
        queue.push_back(root.as_ref().unwrap().clone()); // 将根节点加入队列
        for children in vec[1..].chunks(2) {
            // 对于剩下的节点，每次取两个，即为当前节点的左右子节点
            let mut iter = children.into_iter();
            let parent = queue.pop_front().unwrap(); // 取出队列中的第一个节点，即为当前节点的父节点
            if let Some(Some(v)) = iter.next() {
                // 如果有左子节点，则新建一个节点，并将其加入队列和父节点的 left 字段
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if let Some(Some(v)) = iter.next() {
                // 如果有右子节点，同上
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
        root // 返回根节点
    } else {
        None
    }
}

// 将二叉树转换为向量
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

// 将二叉树转换为向量并排序
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

// 宏定义，用于创建二叉树
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
