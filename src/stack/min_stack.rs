// 最小栈
// https://leetcode.cn/problems/min-stack/
// INLINE  ../../images/heap/min_stack.jpeg
// 解题思路：辅助栈

pub struct MinStack {
    stack: Vec<i32>,     // 原始栈
    min_stack: Vec<i32>, // 辅助栈，用于存储最小元素
}

/**
 * `&self` 表示该方法获取一个不可变引用。
 * 如果需要获取一个可变引用，请改为 `&mut self`。
 */
impl MinStack {
    pub fn new() -> Self {
        // 构造函数
        MinStack {
            stack: Vec::new(),     // 初始化原始栈
            min_stack: Vec::new(), // 初始化辅助栈
        }
    }

    pub fn push(&mut self, val: i32) {
        // 入栈操作，需要传入可变引用
        self.stack.push(val); // 先将元素加入原始栈
                              // 判断辅助栈传入的元素是否小于等于辅助栈栈顶元素
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val); // 如果小于等于，则将元素加入辅助栈，保持辅助栈栈顶元素最小
        }
    }

    pub fn pop(&mut self) {
        // 出栈操作，需要传入可变引用
        if self.stack.is_empty() {
            // 如果原始栈为空，则直接返回
            return;
        }
        if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
            // 如果原始栈顶元素等于辅助栈栈顶元素
            self.min_stack.pop(); // 则将辅助栈栈顶元素出栈
        }
    }

    pub fn top(&self) -> i32 {
        // 获取原始栈栈顶元素，不需要传入可变引用
        return *self.stack.last().unwrap(); // 返回原始栈的栈顶元素
    }

    pub fn get_min(&self) -> i32 {
        // 获取辅助栈栈顶元素，不需要传入可变引用
        return *self.min_stack.last().unwrap(); // 返回辅助栈的栈顶元素
    }
}
