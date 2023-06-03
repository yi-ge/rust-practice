// 差值数组不同的字符串
// https://leetcode.cn/problems/odd-string-difference
// INLINE  ../../images/array/odd_string_difference.jpeg

pub struct Solution;

impl Solution {
    // 获取字符串中相邻字符的差值数组
    pub fn get(word: &str) -> Vec<i32> {
        let chars: Vec<char> = word.chars().collect();
        let mut diff: Vec<i32> = vec![];
        for i in 0..chars.len() - 1 {
            diff.push((chars[i + 1] as i32) - (chars[i] as i32));
        }
        diff
    }

    // 找出差值数组不同的字符串
    pub fn odd_string(words: Vec<String>) -> String {
        let diff0 = Solution::get(&words[0]); // 获取第一个字符串的差值数组
        let diff1 = Solution::get(&words[1]); // 获取第二个字符串的差值数组
        if diff0 == diff1 {
            // 如果第一个字符串和第二个字符串的差值数组相同
            for i in 2..words.len() {
                // 遍历剩余的字符串
                if diff0 != Solution::get(&words[i]) {
                    // 如果当前字符串的差值数组和前两个字符串的差值数组不同
                    return words[i].clone(); // 返回当前字符串
                }
            }
        }
        // 打印结果
        // println!("diff0: {:?}", diff0);
        if diff0 == Solution::get(&words[2]) {
            // 如果第三个字符串的差值数组和第一个字符串的差值数组相同
            words[1].clone() // 返回第二个字符串
        } else {
            // 否则
            words[0].clone() // 返回第一个字符串
        }
    }
}
