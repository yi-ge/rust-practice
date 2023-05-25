// 差值数组不同的字符串
// https://leetcode.cn/problems/odd-string-difference
// INLINE  ../../images/array/odd_string_difference.jpeg

pub struct Solution;

impl Solution {
    pub fn get(word: &str) -> Vec<i32> {
        let chars: Vec<char> = word.chars().collect();
        let mut diff: Vec<i32> = vec![];
        for i in 0..chars.len() - 1 {
            diff.push((chars[i + 1] as i32) - (chars[i] as i32));
        }
        diff
    }

    pub fn odd_string(words: Vec<String>) -> String {
        let diff0 = Solution::get(&words[0]);
        let diff1 = Solution::get(&words[1]);
        if diff0 == diff1 {
            for i in 2..words.len() {
                if diff0 != Solution::get(&words[i]) {
                    return words[i].clone();
                }
            }
        }
        // 打印结果
        // println!("diff0: {:?}", diff0);
        if diff0 == Solution::get(&words[2]) {
            words[1].clone()
        } else {
            words[0].clone()
        }
    }
}
