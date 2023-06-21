// 黑白翻转棋
// https://leetcode.cn/problems/fHi6rV
// INLINE  ../../images/search/fHi6rV.jpeg

use std::{cmp::max, collections::VecDeque};

pub struct Solution;

impl Solution {
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        let m = chessboard.len();
        let n = chessboard[0].len();
        let dx: [isize; 8] = [1, -1, 0, 0, 1, 1, -1, -1];
        let dy: [isize; 8] = [0, 0, 1, -1, 1, -1, 1, -1];

        let mut res = 0;
        let mut cnt;

        for i in 0..m {
            for j in 0..n {
                if chessboard[i].chars().nth(j).unwrap() == '.' {
                    let mut g = chessboard.clone(); // Copy the initial chessboard
                    g[i].remove(j);
                    g[i].insert(j, 'X');
                    cnt = 0;
                    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                    q.push_back((i, j));
                    while let Some((x, y)) = q.pop_front() {
                        for k in 0..8 {
                            let mut find = 0;
                            let mut nx = (x as isize) + dx[k];
                            let mut ny = (y as isize) + dy[k];
                            while nx >= 0
                                && (nx as usize) < m
                                && ny >= 0
                                && (ny as usize) < n
                                && g[nx as usize].chars().nth(ny as usize).unwrap() != '.'
                            {
                                if g[nx as usize].chars().nth(ny as usize).unwrap() == 'X' {
                                    find = 1;
                                    break;
                                }
                                nx += dx[k];
                                ny += dy[k];
                            }
                            if find == 1 {
                                let mut nx = (x as isize) + dx[k];
                                let mut ny = (y as isize) + dy[k];
                                while nx >= 0
                                    && (nx as usize) < m
                                    && ny >= 0
                                    && (ny as usize) < n
                                    && g[nx as usize].chars().nth(ny as usize).unwrap() != '.'
                                {
                                    if g[nx as usize].chars().nth(ny as usize).unwrap() != 'X' {
                                        g[nx as usize].remove(ny as usize);
                                        g[nx as usize].insert(ny as usize, 'X');
                                        cnt += 1;
                                        q.push_back((nx as usize, ny as usize));
                                    } else {
                                        break;
                                    }
                                    nx += dx[k];
                                    ny += dy[k];
                                }
                            }
                        }
                    }
                    res = max(res, cnt);
                }
            }
        }
        res
    }
}
