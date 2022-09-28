// 第 k 个数
// https://leetcode.cn/problems/get-kth-magic-number-lcci
// INLINE  ../../images/map/get_kth_magic_number_lcci.jpeg
// 解题思路：最小堆。

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub struct Solution;

impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let factors = [3, 5, 7];
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        set.insert(1);
        heap.push(Reverse(1));
        let mut magic = 0i64;
        for _i in 0..k {
            if let Some(Reverse(m)) = heap.pop() {
                magic = m;
                for factor in factors {
                    let next = magic * factor;
                    if !set.contains(&next) {
                        set.insert(next);
                        heap.push(Reverse(next));
                    }
                }
            }
        }

        magic as i32
    }
}
