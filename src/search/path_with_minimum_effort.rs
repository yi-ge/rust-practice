// 最小体力消耗路径
// https://leetcode.cn/problems/path-with-minimum-effort
// INLINE  ../../images/search/path_with_minimum_effort.jpeg
use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let m = heights.len();
        let n = heights[0].len();

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0, 0));

        let mut dist = vec![std::i32::MAX; m * n];
        dist[0] = 0;
        let mut seen = vec![false; m * n];

        while let Some((Reverse(d), x, y)) = heap.pop() {
            let id = x * n + y;
            if seen[id] {
                continue;
            }
            if x == m - 1 && y == n - 1 {
                break;
            }
            seen[id] = true;
            for dir in &[[0, 1], [0, -1], [1, 0], [-1, 0]] {
                let nx = x as i32 + dir[0];
                let ny = y as i32 + dir[1];
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let effort = max(d, (heights[x][y] - heights[nx][ny]).abs());
                    if effort < dist[nx * n + ny] {
                        dist[nx * n + ny] = effort;
                        heap.push((Reverse(effort), nx, ny));
                    }
                }
            }
        }

        dist[m * n - 1]
    }
}
