use rust_practice::heap::min_stack::MinStack;

#[test]
fn new() {
    // 示例 1:
    // 输入：
    // ["MinStack","push","push","push","getMin","pop","top","getMin"]
    // [[],[-2],[0],[-3],[],[],[],[]]

    // 输出：
    // [null,null,null,null,-3,null,0,-2]

    // 解释：
    // MinStack minStack = new MinStack();
    // minStack.push(-2);
    // minStack.push(0);
    // minStack.push(-3);
    // minStack.getMin();   --> 返回 -3.
    // minStack.pop();
    // minStack.top();      --> 返回 0.
    // minStack.getMin();   --> 返回 -2.

    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}
