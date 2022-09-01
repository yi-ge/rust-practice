// 最小栈
// https://leetcode.cn/problems/min-stack/
// INLINE  ../../images/heap/min_stack.jpeg
// 解题思路：辅助栈

pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) { // 记得改&mut
        self.stack.push(val);
        // 判断辅助栈传入的元素是否小于等于辅助栈栈顶元素
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val); // 保持辅助栈栈顶元素最小
        }
    }

    pub fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }
    }

    pub fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }

    pub fn get_min(&self) -> i32 {
        return *self.min_stack.last().unwrap();
    }
}
