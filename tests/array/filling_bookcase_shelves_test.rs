use rust_practice::array::filling_bookcase_shelves::Solution;

#[test]
fn min_height_shelves() {
    // 示例 1：
    let books = vec![
        vec![1, 1],
        vec![2, 3],
        vec![2, 3],
        vec![1, 1],
        vec![1, 1],
        vec![1, 1],
        vec![1, 2],
    ];
    let shelf_width = 4;
    assert_eq!(Solution::min_height_shelves(books, shelf_width), 6);

    // 示例 2:
    // 输入: books = [[1,3],[2,4],[3,2]], shelfWidth = 6
    // 输出: 4
    let books = vec![vec![1, 3], vec![2, 4], vec![3, 2]];
    let shelf_width = 6;
    assert_eq!(Solution::min_height_shelves(books, shelf_width), 4);
}
