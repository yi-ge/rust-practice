// 树节点的第 K 个祖先
// https://leetcode.cn/problems/kth-ancestor-of-a-tree-node
// INLINE  ../../images/tree/kth_ancestor_of_a_tree_node.jpeg

const TREE_ANCESTOR_LOG: usize = 16;

pub struct TreeAncestor {
    ancestors: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        // ancestors[i][j] 表示第 2^j 个祖先
        // ancestors[i][0] 表示第 1 个祖先，也就是父节点
        let mut ancestors = vec![vec![-1; TREE_ANCESTOR_LOG]; n as usize];
        for i in 0..n as usize {
            ancestors[i][0] = parent[i];
        }
        for j in 1..TREE_ANCESTOR_LOG {
            for i in 0..n as usize {
                if ancestors[i][j - 1] != -1 {
                    ancestors[i][j] = ancestors[ancestors[i][j - 1] as usize][j - 1];
                }
            }
        }
        TreeAncestor { ancestors }
    }

    pub fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node as usize;
        for j in 0..TREE_ANCESTOR_LOG {
            if (k >> j) & 1 == 1 {
                node = self.ancestors[node][j] as usize;
                if node == usize::MAX {
                    return -1;
                }
            }
        }
        node as i32
    }
}

// /**
//  * Your TreeAncestor object will be instantiated and called as such:
//  * let obj = TreeAncestor::new(n, parent);
//  * let ret_1: i32 = obj.get_kth_ancestor(node, k);
//  */
