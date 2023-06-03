// 单字符重复子串的最大长度
// https://leetcode.cn/problems/swap-for-longest-repeated-character-substring
// INLINE  ../../images/map/swap_for_longest_repeated_character_substring.jpeg

// use std::convert::TryInto;
pub struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let text = text.chars().collect::<Vec<char>>(); // 将字符串转换为字符数组
        let (mut l, n, mut ans) = (0, text.len(), 0); // 初始化左指针l为0，长度n为字符数组长度，最大长度ans为0
        while l < n {
            // 当左指针小于长度时循环
            let mut r = l + 1; // 初始化右指针r为l+1
            while r < n && text[r] == text[l] {
                // 如果r小于长度且r所指向的字符与l所指向的字符相同
                r += 1; // 将r向右移动
            } // 找到第一个与l所指向字符不同的位置
            if r == n {
                // 如果r指向末尾位置
                return ans.max(
                    // 返回ans与以下值的较大者
                    (r - l + if text[0..l].contains(&text[l]) { 1 } else { 0 }) // 计算子串长度，如果字符数组前l个字符中包含l所指向的字符，则长度加1
                        .try_into() // 转换为i32类型
                        .unwrap(), // 如果转换失败，则抛出panic异常
                );
            } // 处理字符串末尾的情况
            let l_ = r; // 将l_赋值为r
            r += 1; // 将r向右移动
            while r < n && text[r] == text[l] {
                // 如果r小于长度且r所指向的字符与l所指向的字符相同
                r += 1; // 将r向右移动
            } // 找到第二个与l所指向字符不同的位置
            if r - l > ans.try_into().unwrap() {
                // 如果子串长度大于ans
                ans = ans.max(
                    // 更新ans为以下值与ans的较大者
                    (r - l // 计算子串长度
                        - if !text[0..l].contains(&text[l]) && !text[r..n].contains(&text[l]) { // 如果l之前的字符数组中不包含l所指向的字符且r之后的字符数组中不包含l所指向的字符
                            1 // 则长度减1
                        } else {
                            0 // 否则长度不变
                        })
                    .try_into() // 转换为i32类型
                    .unwrap(),
                );
            } // 处理子串长度大于ans的情况
            l = l_; // 将l赋值为l_
        }
        ans as i32 // 返回ans
    }
}
