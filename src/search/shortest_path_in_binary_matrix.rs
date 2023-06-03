// 二进制矩阵中的最短路径
// https://leetcode.cn/problems/shortest-path-in-binary-matrix
// INLINE  ../../images/search/shortest_path_in_binary_matrix.jpeg

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        // 如果起点为障碍物，则无法到达终点，返回-1
        if grid[0][0] == 1 {
            return -1;
        }

        let n = grid.len();
        let mut dist = vec![vec![i32::MAX; n]; n]; // 记录到每个点的最短距离
        let mut q = VecDeque::new();
        q.push_back((0, 0)); // 将起点加入队列
        dist[0][0] = 1; // 起点到起点的距离为1

        // 广度优先搜索
        while let Some((x, y)) = q.pop_front() {
            if x == n - 1 && y == n - 1 {
                // 找到终点，返回最短距离
                return dist[x][y];
            }
            for dx in vec![-1, 0, 1].iter() {
                for dy in vec![-1, 0, 1].iter() {
                    // 枚举八个方向
                    let nx = x as isize + *dx;
                    let ny = y as isize + *dy;
                    if nx < 0 || nx >= n as isize || ny < 0 || ny >= n as isize {
                        // 越界，跳过
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 1 || dist[nx][ny] <= dist[x][y] + 1 {
                        // 障碍物或者到达该点的距离不是最短距离，跳过
                        continue;
                    }
                    dist[nx][ny] = dist[x][y] + 1; // 更新到该点的最短距离
                    q.push_back((nx, ny)); // 将该点加入队列
                }
            }
        }
        -1 // 没有找到终点，返回-1
    }
}
