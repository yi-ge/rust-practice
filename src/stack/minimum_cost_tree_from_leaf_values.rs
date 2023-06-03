// 叶值的最小代价生成树
// https://leetcode.cn/problems/minimum-cost-tree-from-leaf-values
// INLINE  ../../images/stack/minimum_cost_tree_from_leaf_values.jpeg

pub struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        // 初始化一个栈，压入一个最大值
        let mut stack = vec![i32::MAX];
        // 初始化结果值
        let mut res = 0;
        // 遍历数组中的每个元素
        for &num in arr.iter() {
            // 如果栈顶元素小于等于当前元素
            while stack.last().unwrap() <= &num {
                // 弹出栈顶元素
                let mid = stack.pop().unwrap();
                // 将结果值加上栈顶元素和栈顶的前一个元素中较小值乘以中间元素
                res += mid * std::cmp::min(stack.last().unwrap(), &num);
            }
            // 将当前元素压入栈中
            stack.push(num);
        }
        // 当栈中元素数量大于2时
        while stack.len() > 2 {
            // 将结果值加上栈顶元素和栈顶的前一个元素的乘积
            res += stack.pop().unwrap() * stack.last().unwrap();
        }
        // 返回结果值
        res
    }
}
