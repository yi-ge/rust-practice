// 受标签影响的最大值
// https://leetcode.cn/problems/largest-values-from-labels
// INLINE  ../../images/sort/largest_values_from_labels.jpeg

pub struct Solution;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut values = values;
        let labels = labels;
        let mut num_wanted = num_wanted;
        let use_limit = use_limit;
        let mut result = 0;
        let mut map = std::collections::HashMap::new();
        let mut index = 0;
        while num_wanted > 0 && index < values.len() {
            let mut max = 0;
            let mut max_index = 0;
            for i in 0..values.len() {
                if values[i] > max {
                    max = values[i];
                    max_index = i;
                }
            }
            if map.contains_key(&labels[max_index]) {
                if map.get(&labels[max_index]).unwrap() < &use_limit {
                    result += max;
                    map.insert(labels[max_index], map.get(&labels[max_index]).unwrap() + 1);
                    num_wanted -= 1;
                }
            } else {
                result += max;
                map.insert(labels[max_index], 1);
                num_wanted -= 1;
            }
            values[max_index] = 0;
            index += 1;
        }
        result
    }
}
