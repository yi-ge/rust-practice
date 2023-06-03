// 第 k 个数
// https://leetcode.cn/problems/get-kth-magic-number-lcci
// INLINE  ../../images/map/get_kth_magic_number_lcci.jpeg
// 解题思路：最小堆。

use std::{
    cmp::Reverse,                       // 导入 Reverse 结构体，用于实现最小堆
    collections::{BinaryHeap, HashSet}, // 导入 BinaryHeap 和 HashSet
};

pub struct Solution;

impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let factors = [3, 5, 7]; // 定义因子数组
        let mut heap = BinaryHeap::new(); // 定义最小堆
        let mut set = HashSet::new(); // 定义哈希集合
        set.insert(1); // 将 1 加入哈希集合
        heap.push(Reverse(1)); // 将 1 加入最小堆
        let mut magic = 0i64; // 定义变量 magic，用于存储堆顶元素
        for _i in 0..k {
            if let Some(Reverse(m)) = heap.pop() {
                // 取出最小堆的堆顶元素
                magic = m; // 将堆顶元素赋值给 magic 变量
                for factor in factors {
                    // 遍历因子数组
                    let next = magic * factor; // 计算下一个魔术数字
                    if !set.contains(&next) {
                        // 如果下一个魔术数字不在哈希集合中
                        set.insert(next); // 将下一个魔术数字加入哈希集合
                        heap.push(Reverse(next)); // 将下一个魔术数字加入最小堆
                    }
                }
            }
        }

        magic as i32 // 返回第 k 个魔术数字
    }
}
