// 二进制矩阵中的最短路径
// https://leetcode.cn/problems/shortest-path-in-binary-matrix
// INLINE  ../../images/search/shortest_path_in_binary_matrix.jpeg

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }

        let n = grid.len();
        let mut dist = vec![vec![i32::MAX; n]; n];
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        dist[0][0] = 1;

        while let Some((x, y)) = q.pop_front() {
            if x == n - 1 && y == n - 1 {
                return dist[x][y];
            }
            for dx in vec![-1, 0, 1].iter() {
                for dy in vec![-1, 0, 1].iter() {
                    let nx = x as isize + *dx;
                    let ny = y as isize + *dy;
                    if nx < 0 || nx >= n as isize || ny < 0 || ny >= n as isize {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 1 || dist[nx][ny] <= dist[x][y] + 1 {
                        continue;
                    }
                    dist[nx][ny] = dist[x][y] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
        -1
    }
}
