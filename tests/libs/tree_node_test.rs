use std::{cell::RefCell, rc::Rc};

use rust_practice::{
    libs::tree_node::{tree_node_to_vec, vec_to_tree_node, TreeNode},
    tree,
};

#[test]
fn test_tree_node_new() {
    let node = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    assert_eq!(node.is_some(), true);
    if let Some(n) = node {
        assert_eq!(n.borrow().val, 1);
        assert_eq!(n.borrow().left == None, true);
    }
}

#[test]
fn test_vec_to_tree_node() {
    // [6, 3, 5, 1, 2, N, 4], N: None
    //
    //           6
    //          / \
    //         3   5
    //        / \   \
    //       1   2   4
    let node1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    let node2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    })));
    let node3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: node1,
        right: node2,
    })));
    let node4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    })));
    let node5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: node4,
    })));
    let node6 = Some(Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: node3,
        right: node5,
    })));
    assert_eq!(
        vec_to_tree_node(&vec![
            Some(6),
            Some(3),
            Some(5),
            Some(1),
            Some(2),
            None,
            Some(4),
        ]),
        node6,
    );

    // [1, NULL, 2, 3], NULL: None
    //
    //           1
    //          / \
    //         N   2
    //            / \
    //           3   N
    let node2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let node1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: node2,
        right: None,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: node1,
    })));
    assert_eq!(
        vec_to_tree_node(&vec![Some(1), None, Some(2), Some(3)]),
        root,
    );
}

#[test]
fn test_tree_node_to_vec() {
    let vec = vec![Some(6), Some(3), Some(5), Some(1), Some(2), None, Some(4)];
    assert_eq!(tree_node_to_vec(vec_to_tree_node(&vec)), vec);

    let vec = vec![Some(1), None, Some(2), Some(3)];
    assert_eq!(tree_node_to_vec(vec_to_tree_node(&vec)), vec);
}

#[test]
fn test_macro_rules_tree() {
    let tree = tree![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vec = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
        Some(10),
    ];
    assert_eq!(tree_node_to_vec(tree), vec);
}
