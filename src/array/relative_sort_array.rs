// 数组的相对排序
// https://leetcode.cn/problems/relative-sort-array/
// INLINE  ../../images/array/relative_sort_array.jpeg

use std::collections::HashMap; // 引入HashMap

pub struct Solution; // 定义Solution结构体

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let map = arr2
            .into_iter() // 转为迭代器
            .enumerate() // 枚举元素
            .map(|(i, x)| (x, i)) // 将元素与其下标组成元组
            .collect::<HashMap<i32, usize>>(); // 转为HashMap
        arr1.sort_by_key(|k| map.get(k).cloned().unwrap_or(1000 + *k as usize)); // 根据map的value排序
        arr1 // 返回排序后的arr1
    }
}
