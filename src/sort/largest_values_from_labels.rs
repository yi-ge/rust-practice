// 受标签影响的最大值
// https://leetcode.cn/problems/largest-values-from-labels
// INLINE  ../../images/sort/largest_values_from_labels.jpeg

pub struct Solution;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>, // 一个整数数组，表示每个元素的值
        labels: Vec<i32>, // 一个整数数组，表示每个元素对应的标签
        num_wanted: i32,  // 要选择的元素个数
        use_limit: i32,   // 每个标签最多选择的元素个数
    ) -> i32 {
        let mut values = values; // 可变的值数组
        let labels = labels; // 不可变的标签数组
        let mut num_wanted = num_wanted; // 可变的要选择的元素个数
        let use_limit = use_limit; // 不可变的每个标签最多选择的元素个数
        let mut result = 0; // 选择的元素值之和
        let mut map = std::collections::HashMap::new(); // 标签计数器
        let mut index = 0; // 当前遍历的元素下标
        while num_wanted > 0 && index < values.len() {
            let mut max = 0; // 当前未选择的元素中的最大值
            let mut max_index = 0; // 当前未选择的元素中的最大值下标
            for i in 0..values.len() {
                // 遍历未选择的元素
                if values[i] > max {
                    // 如果当前元素值大于最大值
                    max = values[i]; // 更新最大值
                    max_index = i; // 更新最大值下标
                }
            }
            if map.contains_key(&labels[max_index]) {
                // 如果标签计数器中已经有了当前元素的标签
                if map.get(&labels[max_index]).unwrap() < &use_limit {
                    // 如果当前标签选择的元素个数还没有达到上限
                    result += max; // 选择当前元素
                    map.insert(labels[max_index], map.get(&labels[max_index]).unwrap() + 1); // 标签计数器中对应标签的计数器加1
                    num_wanted -= 1; // 要选择的元素个数减1
                }
            } else {
                // 如果标签计数器中没有当前元素的标签
                result += max; // 选择当前元素
                map.insert(labels[max_index], 1); // 标签计数器中对应标签的计数器置为1
                num_wanted -= 1; // 要选择的元素个数减1
            }
            values[max_index] = 0; // 将当前元素值置为0，表示已经选择过了
            index += 1; // 当前遍历的元素下标加1
        }
        result // 返回选择的元素值之和
    }
}
