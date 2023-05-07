// 推箱子
// https://leetcode.cn/problems/minimum-moves-to-move-a-box-to-their-target-location
// INLINE  ../../images/search/minimum_moves_to_move_a_box_to_their_target_location.jpeg

pub struct Solution;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut sx = 0;
        let mut sy = 0;
        let mut bx = 0;
        let mut by = 0;

        for x in 0..m {
            for y in 0..n {
                match grid[x][y] {
                    'S' => {
                        sx = x;
                        sy = y;
                    }
                    'B' => {
                        bx = x;
                        by = y;
                    }
                    _ => {}
                }
            }
        }

        let d = vec![0, -1, 0, 1, 0];

        let ok = |x: usize, y: usize| -> bool {
            return x < m && y < n && grid[x][y] != '#';
        };

        let mut dp = vec![vec![std::i32::MAX; m * n]; m * n];
        let mut q: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();

        dp[sx * n + sy][bx * n + by] = 0;
        q.push_back((sx * n + sy, bx * n + by));
        while !q.is_empty() {
            let mut q1: std::collections::VecDeque<(usize, usize)> =
                std::collections::VecDeque::new();
            while !q.is_empty() {
                let (s1, b1) = q.pop_front().unwrap();
                let (sx1, sy1) = (s1 / n, s1 % n);
                let (bx1, by1) = (b1 / n, b1 % n);

                if grid[bx1][by1] == 'T' {
                    return dp[s1][b1];
                }

                for i in 0..4 {
                    let (sx2, sy2) = (sx1 as isize + d[i], sy1 as isize + d[i + 1]);
                    if !ok(sx2 as usize, sy2 as usize) {
                        continue;
                    }
                    let s2 = (sx2 * n as isize + sy2) as usize;

                    if bx1 == sx2 as usize && by1 == sy2 as usize {
                        let (bx2, by2) = (bx1 as isize + d[i], by1 as isize + d[i + 1]);
                        if !ok(bx2 as usize, by2 as usize)
                            || dp[s2][(bx2 * n as isize + by2) as usize] <= dp[s1][b1] + 1
                        {
                            continue;
                        }
                        dp[s2][(bx2 * n as isize + by2) as usize] = dp[s1][b1] + 1;
                        q1.push_back((s2, (bx2 * n as isize + by2) as usize));
                    } else {
                        if dp[s2][b1] <= dp[s1][b1] {
                            continue;
                        }
                        dp[s2][b1] = dp[s1][b1];
                        q.push_back((s2, b1));
                    }
                }
            }
            q = q1;
        }

        -1
    }
}
