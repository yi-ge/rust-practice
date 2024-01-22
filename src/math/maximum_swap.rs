// 最大交换
// https://leetcode.cn/problems/maximum-swap
// INLINE  ../../images/math/maximum_swap.jpeg

pub struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut s: Vec<char> = num.to_string().chars().collect();
        let n = s.len();
        let mut max_idx = n - 1;
        let mut idx1 = None;
        let mut idx2 = None;

        for i in (0..n).rev() {
            if s[i] > s[max_idx] {
                max_idx = i;
            } else if s[i] < s[max_idx] {
                idx1 = Some(i);
                idx2 = Some(max_idx);
            }
        }

        if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
            s.swap(idx1, idx2);
            s.iter().collect::<String>().parse::<i32>().unwrap_or(num)
        } else {
            num
        }
    }
}
