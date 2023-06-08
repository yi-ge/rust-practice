// 铺瓷砖
// https://leetcode.cn/problems/tiling-a-rectangle-with-the-fewest-squares
// INLINE  ../../images/other/tiling_a_rectangle_with_the_fewest_squares.jpeg

pub struct Solution;

impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let mut ans = n * m; // 初始化答案为 n * m
        let mut filled = vec![0; n as usize]; // 初始化 filled 数组，用于记录每个位置是否被铺上瓷砖
        Self::dfs(0, 0, 0, n, m, &mut ans, &mut filled); // 开始深度优先搜索
        ans // 返回答案
    }

    fn dfs(i: i32, j: i32, t: i32, n: i32, m: i32, ans: &mut i32, filled: &mut Vec<i32>) {
        let (mut i, mut j) = (i, j); // 初始化 i 和 j
        if j == m {
            // 如果 j 等于 m，说明当前行已经铺满了，需要换行
            i += 1; // 换行
            j = 0; // 从第一列开始
        }
        if i == n {
            // 如果 i 等于 n，说明所有位置都已经铺满了，更新答案并返回
            *ans = (*ans).min(t); // 更新答案
            return; // 返回
        }
        if filled[i as usize] >> j & 1 == 1 {
            // 如果当前位置已经被铺上瓷砖，跳过
            Self::dfs(i, j + 1, t, n, m, ans, filled); // 继续搜索下一个位置
        } else if t + 1 < *ans {
            // 如果当前铺瓷砖的数量加一小于答案，继续搜索
            let mut r = 0; // 初始化 r
            let mut c = 0; // 初始化 c
            for k in i..n {
                // 从当前行开始向下遍历
                if filled[k as usize] >> j & 1 == 1 {
                    // 如果当前位置已经被铺上瓷砖，跳出循环
                    break;
                }
                r += 1; // 更新 r
            }
            for k in j..m {
                // 从当前列开始向右遍历
                if filled[i as usize] >> k & 1 == 1 {
                    // 如果当前位置已经被铺上瓷砖，跳出循环
                    break;
                }
                c += 1; // 更新 c
            }
            let mx = r.min(c); // 取 r 和 c 的最小值
            for w in 1..=mx {
                // 枚举铺瓷砖的大小
                let old_filled = filled.clone(); // 备份 filled 数组
                for x in i..i + w {
                    // 铺瓷砖
                    for y in j..j + w {
                        filled[x as usize] |= 1 << y;
                    }
                }
                Self::dfs(i, j + w, t + 1, n, m, ans, filled); // 继续搜索下一个位置
                *filled = old_filled; // 恢复 filled 数组
            }
        }
    }
}
