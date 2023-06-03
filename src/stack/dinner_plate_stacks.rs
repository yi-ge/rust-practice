// 餐盘栈
// https://leetcode.cn/problems/dinner-plate-stacks
// INLINE  ../../images/stack/dinner_plate_stacks.jpeg
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct DinnerPlates {
    stack_map: HashMap<i32, Vec<i32>>,         // 存储每个栈的元素
    available_index: BinaryHeap<Reverse<i32>>, // 存储可用的栈编号，使用最小堆来获取最小的可用栈编号
    max_index: i32,                            // 当前最大的栈编号
    capacity: usize,                           // 每个栈的最大容量
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
            // 如果有可用的栈，则将元素加入该栈
            self.max_index = self.max_index.max(target); // 更新最大的栈编号
            let list = self.stack_map.entry(target).or_insert_with(Vec::new); // 获取该栈的元素，如果该栈不存在则创建一个新的空向量
            list.push(val); // 将元素加入该栈
        } else {
            // 如果没有可用的栈，则创建一个新的栈
            let list = self
                .stack_map
                .entry(self.max_index)
                .or_insert_with(Vec::new); // 获取最大的栈的元素，如果该栈不存在则创建一个新的空向量
            if list.len() != self.capacity {
                // 如果该栈未满，则将元素加入该栈
                list.push(val);
            } else {
                // 如果该栈已满，则创建一个新的栈，并将元素加入该栈
                self.max_index += 1;
                self.stack_map.insert(self.max_index, vec![val]);
            }
        }
    }

    pub fn pop(&mut self) -> i32 {
        // 弹出最右边的元素
        while let Some(list) = self.stack_map.get_mut(&self.max_index) {
            // 从最大的栈开始，遍历每个栈，直到找到非空的栈
            if let Some(num) = list.pop() {
                // 如果该栈非空，则将最右边的元素弹出
                self.available_index.push(Reverse(self.max_index)); // 将该栈的编号加入可用栈的堆中
                return num;
            }
            self.max_index -= 1; // 如果该栈为空，则将最大的栈编号向左移动一位
        }
        self.max_index = 0; // 如果找不到非空的栈，则将最大的栈编号重置为0
        -1 // 返回-1表示没有找到可弹出的元素
    }

    pub fn pop_at_stack(&mut self, index: i32) -> i32 {
        // 弹出指定栈的最右边的元素
        if let Some(list) = self.stack_map.get_mut(&index) {
            // 如果该栈存在
            if let Some(num) = list.pop() {
                // 将该栈的最右边的元素弹出
                self.available_index.push(Reverse(index)); // 将该栈的编号加入可用栈的堆中
                return num;
            }
        }
        #[cfg_attr(tarpaulin, skip)]
        -1 // 如果该栈不存在或为空，则返回-1表示没有找到可弹出的元素
    }
}
