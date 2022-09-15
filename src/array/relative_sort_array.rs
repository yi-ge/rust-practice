// 数组的相对排序
// https://leetcode.cn/problems/relative-sort-array/
// INLINE  ../../images/array/relative_sort_array.jpeg

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let map = arr2
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<i32, usize>>();
        arr1.sort_by_key(|k| map.get(k).cloned().unwrap_or(1000 + *k as usize));
        arr1
    }
}
