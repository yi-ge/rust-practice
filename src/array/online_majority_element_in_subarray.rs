// 子数组中占绝大多数的元素
// https://leetcode.cn/problems/online-majority-element-in-subarray
// INLINE  ../../images/array/online_majority_element_in_subarray.jpeg

use std::collections::HashMap;

use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng, thread_rng};

// 二分查找，返回第一个大于等于value的元素下标
fn lower_bound(slice: &[i32], value: i32) -> usize {
    match slice.binary_search(&value) {
        Ok(mut index) => {
            while index > 0 && slice[index - 1] == value {
                index -= 1;
            }
            index
        }
        Err(index) => index,
    }
}

// 二分查找，返回第一个大于value的元素下标
fn upper_bound(slice: &[i32], value: i32) -> usize {
    match slice.binary_search(&value) {
        Ok(mut index) => {
            while index + 1 < slice.len() && slice[index + 1] == value {
                index += 1;
            }
            index + 1
        }
        Err(index) => index,
    }
}

pub struct MajorityChecker {
    arr: Vec<i32>,               // 数组
    loc: HashMap<i32, Vec<i32>>, // 哈希表，存放每个元素的下标列表
    gen: ThreadRng,              // 随机数生成器
}

impl MajorityChecker {
    // 构造函数，初始化数组和哈希表
    pub fn new(arr: Vec<i32>) -> Self {
        let mut loc = HashMap::new();
        for i in 0..arr.len() {
            loc.entry(arr[i]).or_insert(Vec::new()).push(i as i32);
        }
        MajorityChecker {
            arr,
            loc,
            gen: thread_rng(),
        }
    }

    // 查询函数，随机找到一个元素，判断该元素在[left, right]范围内出现的次数是否大于等于threshold
    // 如果大于等于threshold，返回该元素
    // 如果小于threshold但是该元素在[left, right]范围内出现的次数大于length的一半，返回-1
    // 如果该元素在[left, right]范围内出现的次数小于等于length的一半，返回-1
    pub fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        let length = right - left + 1;
        let dis = Uniform::from(left..=right);

        for _ in 0..20 {
            let x = self.arr[dis.sample(&mut self.gen) as usize]; // 随机找到一个元素
            let pos: &[i32] = self.loc.get(&x).unwrap(); // 获取该元素在原数组中的下标列表
            let occ = (upper_bound(pos, right) - lower_bound(pos, left)) as i32; // 计算该元素在[left, right]范围内出现的次数
            if occ >= threshold {
                // 如果该元素在[left, right]范围内出现的次数大于等于threshold，返回该元素
                return x;
            } else if occ * 2 >= length {
                // 如果该元素在[left, right]范围内出现的次数大于length的一半，返回-1
                return -1;
            }
        }
        -1 // 如果随机20次都没有找到符合条件的元素，返回-1
    }
}
