// 距离相等的条形码
// https://leetcode.cn/problems/distant-barcodes
// INLINE  ../../images/sort/distant_barcodes.jpeg
// 最大堆
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        for b in barcodes {
            *count.entry(b).or_insert(0) += 1;
        }
        let mut q = BinaryHeap::new();
        for (b, c) in count {
            q.push((c, b));
        }
        let mut res = Vec::new();
        while !q.is_empty() {
            let (c, b) = q.pop().unwrap();
            if res.is_empty() || *res.last().unwrap() != b {
                res.push(b);
                if c > 1 {
                    q.push((c - 1, b));
                }
            } else {
                // if q.is_empty() {
                //     return res;
                // }
                let (c1, b1) = q.pop().unwrap();
                res.push(b1);
                if c1 > 1 {
                    q.push((c1 - 1, b1));
                }
                q.push((c, b));
            }
        }
        res
    }
}
