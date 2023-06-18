// 统计封闭岛屿的数目
// https://leetcode.cn/problems/number-of-closed-islands
// INLINE  ../../images/search/number_of_closed_islands.jpeg

pub struct Solution;

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        let n = grid[0].len();
        let m = grid.len();

        // 定义函数 dfs 用于检查从 (x,y) 是否可以到达边界
        fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> bool {
            let n = grid[0].len() as i32;
            let m = grid.len() as i32;

            // 如果 (x,y) 超出网格边界，说明可以到达边界
            if x < 0 || y < 0 || x >= m || y >= n {
                return false;
            }
            // 如果 (x,y) 不是陆地，说明可以到达边界
            if grid[x as usize][y as usize] != 0 {
                return true;
            }
            // 标记 (x,y) 为已访问
            grid[x as usize][y as usize] = -1;
            // 检查从 (x,y) 是否可以到达边界
            let ret1 = dfs(grid, x - 1, y);
            let ret2 = dfs(grid, x + 1, y);
            let ret3 = dfs(grid, x, y - 1);
            let ret4 = dfs(grid, x, y + 1);
            // 如果从 (x,y) 可以到达边界，说明不是封闭岛屿
            return ret1 && ret2 && ret3 && ret4;
        }

        // 检查每个网格，找到封闭岛屿
        for i in 0..m {
            for j in 0..n {
                // 如果 (i,j) 是陆地，检查从 (i,j) 是否可以到达边界
                if grid[i][j] == 0 && dfs(&mut grid, i as i32, j as i32) {
                    ans += 1;
                }
            }
        }

        return ans;
    }
}
