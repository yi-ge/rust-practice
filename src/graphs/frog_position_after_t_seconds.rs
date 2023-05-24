// T 秒后青蛙的位置
// https://leetcode.cn/problems/frog-position-after-t-seconds
// INLINE  ../../images/graphs/frog_position_after_t_seconds.jpeg

pub struct Solution;

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut g = vec![vec![]; n as usize + 1];
        for edge in &edges {
            g[edge[0] as usize].push(edge[1]);
            g[edge[1] as usize].push(edge[0]);
        }
        let mut visited = vec![false; n as usize + 1];
        Solution::dfs(&g, &mut visited, 1, t, target)
    }

    fn dfs(g: &[Vec<i32>], visited: &mut [bool], i: i32, t: i32, target: i32) -> f64 {
        let nxt = if i == 1 {
            g[i as usize].len()
        } else {
            g[i as usize].len() - 1
        };

        if t == 0 || nxt == 0 {
            return if i == target { 1.0 } else { 0.0 };
        }

        visited[i as usize] = true;
        let mut ans = 0.0;
        for &j in &g[i as usize] {
            if !visited[j as usize] {
                ans += Solution::dfs(g, visited, j, t - 1, target);
            }
        }
        ans / (nxt as f64)
    }
}
