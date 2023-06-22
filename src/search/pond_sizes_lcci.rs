// 水域大小
// https://leetcode.cn/problems/pond-sizes-lcci
// INLINE  ../../images/search/pond_sizes_lcci.jpeg

pub struct Solution;

impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let m = land.len();
        let n = land[0].len();

        // 定义深度优先搜索函数
        fn dfs(land: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
            let m = land.len();
            let n = land[0].len();

            // 如果越界或已经访问过，返回0
            if x >= m || y >= n || land[x][y] != 0 {
                return 0;
            }

            // 标记为已访问
            land[x][y] = -1;
            let mut res = 1;

            // 递归搜索8个方向
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let x_new = (x as i32) + dx;
                    let y_new = (y as i32) + dy;
                    if x_new >= 0 && y_new >= 0 {
                        res += dfs(land, x_new as usize, y_new as usize);
                    }
                }
            }
            res
        }

        let mut res = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 0 {
                    res.push(dfs(&mut land, i, j));
                }
            }
        }

        res.sort_unstable();
        res
    }
}
