// 最大化网格幸福感
// https://leetcode.cn/problems/maximize-grid-happiness
// INLINE  ../../images/math/maximize_grid_happiness.jpeg

use std::cmp;
pub struct Solution;

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let mut dp = vec![vec![vec![vec![-1; 250]; 7]; 7]; 26];
        let pow1 = 3i32.pow(n as u32);
        let pow2 = 3i32.pow((n - 1) as u32);
        let offset = vec![vec![0, 0, 0], vec![0, -60, -10], vec![0, -10, 40]];

        fn dfs(
            cur: i32,
            a: i32,
            b: i32,
            status: i32,
            dp: &mut Vec<Vec<Vec<Vec<i32>>>>,
            m: i32,
            n: i32,
            pow1: i32,
            pow2: i32,
            offset: &Vec<Vec<i32>>,
        ) -> i32 {
            if cur == m * n {
                return 0;
            }
            if dp[cur as usize][a as usize][b as usize][status as usize] != -1 {
                return dp[cur as usize][a as usize][b as usize][status as usize];
            }
            // let x = cur / n;
            let y = cur % n;
            let first_state = status / pow2;
            let last_state = status % 3;
            let next_status = status * 3 % pow1;
            let mut ans = dfs(cur + 1, a, b, next_status, dp, m, n, pow1, pow2, offset);
            #[allow(unused_assignments)]
            let mut diff = 0;
            if a > 0 {
                diff = 120
                    + offset[1][first_state as usize]
                    + (if y > 0 {
                        offset[1][last_state as usize]
                    } else {
                        0
                    });
                ans = cmp::max(
                    ans,
                    diff + dfs(
                        cur + 1,
                        a - 1,
                        b,
                        next_status + 1,
                        dp,
                        m,
                        n,
                        pow1,
                        pow2,
                        offset,
                    ),
                );
            }
            if b > 0 {
                diff = 40
                    + offset[2][first_state as usize]
                    + (if y > 0 {
                        offset[2][last_state as usize]
                    } else {
                        0
                    });
                ans = cmp::max(
                    ans,
                    diff + dfs(
                        cur + 1,
                        a,
                        b - 1,
                        next_status + 2,
                        dp,
                        m,
                        n,
                        pow1,
                        pow2,
                        offset,
                    ),
                );
            }
            dp[cur as usize][a as usize][b as usize][status as usize] = ans;
            ans
        }

        dfs(
            0,
            introverts_count,
            extroverts_count,
            0,
            &mut dp,
            m,
            n,
            pow1,
            pow2,
            &offset,
        )
    }
}
