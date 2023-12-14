// 用邮票贴满网格图
// https://leetcode.cn/problems/stamping-the-grid
// INLINE  ../../images/array/stamping_the_grid.jpeg

pub struct Solution;

impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let stamp_height = stamp_height as usize;
        let stamp_width = stamp_width as usize;

        // 检查网格中是否有任何空白（0），如果没有，则直接返回 true
        if grid.iter().all(|row| row.iter().all(|&cell| cell == 1)) {
            return true;
        }

        // 如果邮票尺寸大于网格本身，则无法放置邮票
        if stamp_height > m || stamp_width > n {
            return false;
        }

        let mut prefix_sum = vec![vec![0; n + 1]; m + 1];
        let mut diff = vec![vec![0; n + 1]; m + 1];

        // 计算前缀和
        for i in 1..=m {
            for j in 1..=n {
                prefix_sum[i][j] = grid[i - 1][j - 1] + prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                    - prefix_sum[i - 1][j - 1];
            }
        }

        // 使用差分数组记录邮票放置情况
        for i in 1..=m - stamp_height + 1 {
            for j in 1..=n - stamp_width + 1 {
                let x = i + stamp_height - 1;
                let y = j + stamp_width - 1;
                if prefix_sum[x][y] - prefix_sum[x][j - 1] - prefix_sum[i - 1][y]
                    + prefix_sum[i - 1][j - 1]
                    == 0
                {
                    diff[i][j] += 1;
                    if y + 1 <= n {
                        diff[i][y + 1] -= 1;
                    }
                    if x + 1 <= m {
                        diff[x + 1][j] -= 1;
                    }
                    if x + 1 <= m && y + 1 <= n {
                        diff[x + 1][y + 1] += 1;
                    }
                }
            }
        }

        // 更新差分数组并检查是否每个空白格都至少被一个邮票覆盖
        for i in 1..=m {
            for j in 1..=n {
                if i > 1 {
                    diff[i][j] += diff[i - 1][j];
                }
                if j > 1 {
                    diff[i][j] += diff[i][j - 1];
                }
                if i > 1 && j > 1 {
                    diff[i][j] -= diff[i - 1][j - 1];
                }
                if diff[i][j] == 0 && grid[i - 1][j - 1] == 0 {
                    return false;
                }
            }
        }

        true
    }
}
