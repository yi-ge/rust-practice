// 不邻接植花
// https://leetcode.cn/problems/flower-planting-with-no-adjacent
// INLINE  ../../images/graphs/flower_planting_with_no_adjacent.jpeg
// 参考：https://leetcode.cn/problems/flower-planting-with-no-adjacent/solution/rust-shuang-bai-qing-xi-si-lu-by-a-li-ke-si/

pub struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        // 初始化颜色数组
        let mut color = vec![0; n as usize + 1];
        // 初始化邻接表
        let mut adj = vec![vec![]; n as usize + 1];

        // 构建邻接表
        for path in paths {
            let i = path[0] as usize;
            let j = path[1] as usize;
            adj[i].push(j);
            adj[j].push(i);
        }

        // 遍历每个花园
        for i in 1..=n as usize {
            // 初始化可选颜色数组
            let mut valid = [true; 5];
            // 遍历邻居花园，将已经涂过的颜色标记为不可选
            for &j in adj[i].iter() {
                valid[color[j]] = false;
            }

            // 选取第一个可选的颜色
            for k in 1..=4 {
                if valid[k] {
                    color[i] = k;
                    break;
                }
            }
        }

        // 返回花园涂的颜色
        color[1..].iter().map(|x| (*x) as i32).collect()
    }
}
