// 自由之路
// https://leetcode.cn/problems/freedom-trail
// INLINE  ../../images/search/freedom_trail.jpeg

pub struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut idx_list: Vec<Vec<usize>> = vec![vec![]; 26];
        let n = ring.len();
        let m = key.len();
        let mut pc = (ring.chars().next().unwrap() as u8 - b'a') as usize;

        for (i, c) in ring.chars().enumerate() {
            idx_list[(c as u8 - b'a') as usize].push(i);
        }

        let mut dp: Vec<i32> = vec![0x3f3f3f3f; n];
        let mut pre: Vec<i32> = vec![0x3f3f3f3f; n];
        pre[0] = 0;

        for i in 0..m {
            for &idx in &idx_list[key.chars().nth(i).unwrap() as usize - 'a' as usize] {
                for &pi in &idx_list[pc] {
                    dp[idx] = dp[idx].min(
                        pre[pi]
                            + (pi as i32 - idx as i32)
                                .abs()
                                .min(n as i32 - (pi as i32 - idx as i32).abs()),
                    );
                }
            }
            pre = dp.clone();
            dp = vec![0x3f3f3f3f; n];
            pc = key.chars().nth(i).unwrap() as usize - 'a' as usize;
        }

        *pre.iter().min().unwrap() + key.len() as i32
    }
}
