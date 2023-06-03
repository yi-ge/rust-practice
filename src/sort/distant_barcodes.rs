// 距离相等的条形码
// https://leetcode.cn/problems/distant-barcodes
// INLINE  ../../images/sort/distant_barcodes.jpeg
// 最大堆
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        // 建立哈希表统计每个条形码出现的次数
        let mut count = HashMap::new();
        for b in barcodes {
            *count.entry(b).or_insert(0) += 1;
        }
        // 将哈希表中的元素按出现次数从大到小加入最大堆中
        let mut q = BinaryHeap::new();
        for (b, c) in count {
            q.push((c, b));
        }
        let mut res = Vec::new();
        // 取出堆顶元素，加入结果数组中
        // 如果结果数组为空或者堆顶元素与结果数组最后一个元素不同，则继续加入堆顶元素
        // 如果堆顶元素与结果数组最后一个元素相同，则先取出次大的元素加入结果数组，再将堆顶元素加入堆中
        while !q.is_empty() {
            let (c, b) = q.pop().unwrap();
            if res.is_empty() || *res.last().unwrap() != b {
                res.push(b);
                if c > 1 {
                    q.push((c - 1, b));
                }
            } else {
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
