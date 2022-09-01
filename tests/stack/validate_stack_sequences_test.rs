use rust_practice::stack::validate_stack_sequences::Solution;

#[test]
fn validate_stack_sequences() {
    // 示例 1：
    // 输入：pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
    // 输出：true
    // 解释：我们可以按以下顺序执行：
    // push(1), push(2), push(3), push(4), pop() -> 4,
    // push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    assert_eq!(Solution::validate_stack_sequences(pushed, popped), true);

    // 示例 2：
    // 输入：pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
    // 输出：false
    // 解释：1 不能在 2 之前弹出。
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 2, 5, 1, 2];
    assert_eq!(Solution::validate_stack_sequences(pushed, popped), false);
}
