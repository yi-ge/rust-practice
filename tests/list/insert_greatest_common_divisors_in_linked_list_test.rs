use rust_practice::libs::list_node::vec_to_list_node;
use rust_practice::list::insert_greatest_common_divisors_in_linked_list::Solution;

#[test]
fn insert_greatest_common_divisors() {
    // 示例 1：
    // 输入：head = [18,6,10,3]
    // 输出：[18,6,6,2,10,1,3]
    // 解释：第一幅图是一开始的链表，第二幅图是插入新结点后的图（蓝色结点为新插入结点）。
    // - 18 和 6 的最大公约数为 6 ，插入第一和第二个结点之间。
    // - 6 和 10 的最大公约数为 2 ，插入第二和第三个结点之间。
    // - 10 和 3 的最大公约数为 1 ，插入第三和第四个结点之间。
    // 所有相邻结点之间都插入完毕，返回链表。
    let arr = vec![18, 6, 10, 3];
    let head = vec_to_list_node(&arr);
    let res = vec![18, 6, 6, 2, 10, 1, 3];
    let res = vec_to_list_node(&res);
    assert_eq!(Solution::insert_greatest_common_divisors(head), res);

    // 示例 2：
    // 输入：head = [7]
    // 输出：[7]
    // 解释：第一幅图是一开始的链表，第二幅图是插入新结点后的图（蓝色结点为新插入结点）。
    // 没有相邻结点，所以返回初始链表。
    let arr = vec![7];
    let head = vec_to_list_node(&arr);
    let res = vec![7];
    let res = vec_to_list_node(&res);
    assert_eq!(Solution::insert_greatest_common_divisors(head), res);
}
