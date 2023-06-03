// 填充书架
// https://leetcode.cn/problems/filling-bookcase-shelves
// INLINE  ../../images/array/filling_bookcase_shelves.jpeg

pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        // 初始化dp数组，dp[i]表示前i本书放入书架所需要的最小高度
        let mut dp = vec![0; books.len() + 1];
        for i in 1..=books.len() {
            let mut width = books[i - 1][0];
            let mut height = books[i - 1][1];
            // 先假设当前这本书是一层书架的第一本书，高度为当前书的高度
            dp[i] = dp[i - 1] + height;
            // 从后往前遍历之前的书，找到一组能放在同一层的书，使得高度最大
            for j in (1..i).rev() {
                if width + books[j - 1][0] > shelf_width {
                    break;
                }
                width += books[j - 1][0];
                height = height.max(books[j - 1][1]);
                // 如果这组书的宽度小于等于书架的宽度，就可以放在同一层，更新dp数组
                dp[i] = dp[i].min(dp[j - 1] + height);
            }
        }
        dp[books.len()] // 返回放入所有书所需要的最小高度
    }
}
