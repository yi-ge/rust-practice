// 填充书架
// https://leetcode.cn/problems/filling-bookcase-shelves
// INLINE  ../../images/array/filling_bookcase_shelves.jpeg

pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![0; books.len() + 1];
        for i in 1..=books.len() {
            let mut width = books[i - 1][0];
            let mut height = books[i - 1][1];
            dp[i] = dp[i - 1] + height;
            for j in (1..i).rev() {
                if width + books[j - 1][0] > shelf_width {
                    break;
                }
                width += books[j - 1][0];
                height = height.max(books[j - 1][1]);
                dp[i] = dp[i].min(dp[j - 1] + height);
            }
        }
        dp[books.len()]
    }
}
