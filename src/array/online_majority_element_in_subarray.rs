// 子数组中占绝大多数的元素
// https://leetcode.cn/problems/online-majority-element-in-subarray
// INLINE  ../../images/array/online_majority_element_in_subarray.jpeg

use std::collections::HashMap;

use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng, thread_rng};

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
    arr: Vec<i32>,
    loc: HashMap<i32, Vec<i32>>,
    gen: ThreadRng,
}

impl MajorityChecker {
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

    pub fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        let length = right - left + 1;
        let dis = Uniform::from(left..=right);

        for _ in 0..20 {
            let x = self.arr[dis.sample(&mut self.gen) as usize];
            let pos: &[i32] = self.loc.get(&x).unwrap();
            let occ = (upper_bound(pos, right) - lower_bound(pos, left)) as i32;
            if occ >= threshold {
                return x;
            } else if occ * 2 >= length {
                return -1;
            }
        }
        -1
    }
}
