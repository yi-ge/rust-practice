// 餐盘栈
// https://leetcode.cn/problems/dinner-plate-stacks
// INLINE  ../../images/stack/dinner_plate_stacks.jpeg
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct DinnerPlates {
    stack_map: HashMap<i32, Vec<i32>>,
    available_index: BinaryHeap<Reverse<i32>>, // Use a min heap
    max_index: i32,
    capacity: usize,
}

impl DinnerPlates {
    pub fn new(capacity: i32) -> Self {
        Self {
            stack_map: HashMap::new(),
            available_index: BinaryHeap::new(),
            max_index: 0,
            capacity: capacity as usize,
        }
    }

    pub fn push(&mut self, val: i32) {
        if let Some(Reverse(target)) = self.available_index.pop() {
            self.max_index = self.max_index.max(target);
            let list = self.stack_map.entry(target).or_insert_with(Vec::new);
            list.push(val);
        } else {
            let list = self
                .stack_map
                .entry(self.max_index)
                .or_insert_with(Vec::new);
            if list.len() != self.capacity {
                list.push(val);
            } else {
                self.max_index += 1;
                self.stack_map.insert(self.max_index, vec![val]);
            }
        }
    }

    pub fn pop(&mut self) -> i32 {
        while let Some(list) = self.stack_map.get_mut(&self.max_index) {
            if let Some(num) = list.pop() {
                self.available_index.push(Reverse(self.max_index));
                return num;
            }
            self.max_index -= 1;
        }
        self.max_index = 0;
        -1
    }

    pub fn pop_at_stack(&mut self, index: i32) -> i32 {
        if let Some(list) = self.stack_map.get_mut(&index) {
            if let Some(num) = list.pop() {
                self.available_index.push(Reverse(index));
                return num;
            }
        }
        #[cfg_attr(tarpaulin, skip)]
        -1
    }
}
