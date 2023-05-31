// 叶值的最小代价生成树
// https://leetcode.cn/problems/minimum-cost-tree-from-leaf-values
// INLINE  ../../images/stack/minimum_cost_tree_from_leaf_values.jpeg

pub struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut stack = vec![i32::MAX];
        let mut res = 0;
        for &num in arr.iter() {
            while stack.last().unwrap() <= &num {
                let mid = stack.pop().unwrap();
                res += mid * std::cmp::min(stack.last().unwrap(), &num);
            }
            stack.push(num);
        }
        while stack.len() > 2 {
            res += stack.pop().unwrap() * stack.last().unwrap();
        }
        res
    }
}
