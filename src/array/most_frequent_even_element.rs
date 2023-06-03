// 出现最频繁的偶数元素
// https://leetcode.cn/problems/most-frequent-even-element
// INLINE  ../../images/array/most_frequent_even_element.jpeg

pub struct Solution;

impl Solution {
    // 定义一个公有函数most_frequent_even，参数为一个i32类型的Vec，返回值为i32类型
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        // 导入标准库中的BTreeMap结构体
        use std::collections::BTreeMap;
        // 定义一个BTreeMap类型的cnt变量，并初始化为空的BTreeMap
        let mut cnt = BTreeMap::new();
        // 对nums中的每个元素进行迭代
        nums.into_iter().for_each(|num| {
            // 如果当前元素为偶数
            if num % 2 == 0 {
                // 在cnt中增加当前元素对应的值，如果该元素不存在，则初始化为0再增加
                *cnt.entry(num).or_default() += 1
            }
        });
        // 对cnt进行迭代，从大到小排序，取出出现次数最多的元素。如果cnt为空，则返回(-1, -1)
        cnt.into_iter()
            .rev()
            .max_by_key(|&e| e.1)
            .unwrap_or((-1, -1))
            .0
    }
}
